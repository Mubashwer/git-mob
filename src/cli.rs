use crate::coauthor_repo::CoauthorRepo;
use crate::commands::{coauthor::Coauthor, mob::Mob};
use clap::{Parser, Subcommand};
use std::str;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about,
    bin_name = "git mob",
    override_usage = "git mob [COMMAND] [OPTIONS]"
)]
#[command(propagate_version = true, arg_required_else_help = true)]
/// A CLI app which can help users automatically add co-author(s) to git commits
/// for pair/mob programming.
///
/// A user can attribute a git commit to more than one author by adding one or more
/// Co-authored-by trailers to the commit's message.
/// Co-authored commits are visible on GitHub.
/// This app will do the above automatically and also help users store and manage
/// co-authors for pair/mob programming sessions.
///
/// Usage example:
///
/// git mob co-author --add lm "Leo Messi" leo.messi@example.com
///
/// git pair with
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    mob: Mob,
}

#[derive(Subcommand)]
enum Commands {
    /// Add/delete/list co-author(s) from co-author repository
    ///
    /// User must store co-author(s) to co-author repository by using keys
    /// (usually initials) before starting pair/mob programming session(s).
    Coauthor(Coauthor),
}

pub fn run(coauthor_repo: &impl CoauthorRepo) {
    let cli = Cli::parse();

    match &cli.command {
        None => cli.mob.handle(coauthor_repo),
        Some(Commands::Coauthor(coauthor)) => coauthor.handle(coauthor_repo),
    }
}

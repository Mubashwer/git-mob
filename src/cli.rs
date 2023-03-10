use crate::{coauthor_repo::CoauthorRepo, commands::mob::Mob};
use clap::{Parser, Subcommand};
use std::str;

#[derive(Parser)]
#[command(author, version, about, long_about)]
#[command(propagate_version = true, arg_required_else_help = true)]
/// A CLI app which can help users automatically add co-author(s) to git commits
/// for pair/mob programming.
///
/// A user can attribute a git commit to more than one author by adding one or more
/// Co-authored-by trailers to the commit's message.
/// Co-authored commits are visible on GitHub.
///
/// This app will do the above automatically and also help users store and manage
/// co-authors for pair/mob programming sessions.
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    mob: Mob,
}

#[derive(Subcommand)]
enum Commands {}

pub fn run(coauthor_repo: &dyn CoauthorRepo) {
    let cli = Cli::parse();

    match &cli.command {
        None => cli.mob.handle(coauthor_repo),
        Some(_) => todo!(),
    }
}

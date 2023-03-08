mod coauthor_repo;
mod with;

use std::str;

use clap::{Parser, Subcommand};
use coauthor_repo::{CoauthorRepo, GitConfigCoauthorRepo};

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
    /// Sets active co-author(s) for mob programming
    #[arg(short='w', long="with", num_args=0.., value_name="COAUTHOR_KEY")]
    with: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum Commands {}

fn main() {
    let cli = Cli::parse();
    let coauthor_repo: Box<dyn CoauthorRepo> = Box::new(GitConfigCoauthorRepo {});

    match &cli.command {
        None => {
            if cli.with.is_some() {
                with::handle(&coauthor_repo, &cli.with.unwrap())
            }
        }
        Some(_) => todo!(),
    }
}

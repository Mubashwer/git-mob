mod coauthor_repo;
mod handlers;

use std::str;

use clap::{Parser, Subcommand};
use coauthor_repo::{CoauthorRepo, GitConfigCoauthorRepo};
use handlers::with;

#[derive(Parser)]
#[command(
    author = "Mubashwer Salman Khurshid",
    version,
    about = "A CLI app to help you automatically add co-author(s) to commits
during pair/mob programming sessions.",
    long_about = "
You can attribute a git commit to more than one author by adding one or more
Co-authored-by trailers to the commit's message. 
Co-authored commits are visible on GitHub.

This CLI app will make it easy for you to manage pair/mob programming sessions
and automatically add Co-authored-by trailers to the commit's message.
"
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Sets active co-author(s) for the pair/mob programming session",
        long_about = "
Sets active co-author(s) for the pair/mob programming session.
This should be run after at least one co-author(s) has been added.

If no COAUTHOR_KEY arg is provided, a multi-select prompt will be displayed to
aid selection from available co-authors.

If COAUTHOR_KEY(s) are provided, co-author(s) must exist with the given key(s).

Example usage:
$ git pair with lm cr
"
    )]
    With {
        #[arg(help = "Keys used to store co-authors (usually initials).")]
        coauthor_keys: Option<Vec<String>>,
    },
}

fn main() {
    let cli = Cli::parse();
    let coauthor_repo: Box<dyn CoauthorRepo> = Box::new(GitConfigCoauthorRepo {});

    match &cli.command {
        Some(Commands::With { coauthor_keys }) => with::handle(&coauthor_repo, &coauthor_keys),
        None => {}
    };
}

mod coauthor_repo;
mod handlers;

use std::str;

use clap::{Parser, Subcommand};
use coauthor_repo::{CoauthorRepo, GitConfigCoauthorRepo};
use handlers::with;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    With { coauthor_keys: Option<Vec<String>> },
}

fn main() {
    let cli = Cli::parse();
    let coauthor_repo: Box<dyn CoauthorRepo> = Box::new(GitConfigCoauthorRepo {});

    match &cli.command {
        Commands::With { coauthor_keys } => with::handle(coauthor_repo, &coauthor_keys),
    }
}

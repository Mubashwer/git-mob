mod coauthor_repo;

use std::str;

use clap::{Parser, Subcommand};
use coauthor_repo::{CoauthorRepo, GitConfigCoauthorRepo};
use inquire::MultiSelect;

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
        Commands::With { coauthor_keys } => match coauthor_keys {
            Some(keys) => {
                coauthor_repo.deactivate_all();

                let coauthors = keys
                    .into_iter()
                    .map(|key| {
                        let coauthor = coauthor_repo.get(key);
                        coauthor_repo.activate(&coauthor);
                        return coauthor;
                    })
                    .collect::<Vec<String>>();

                println!("Active co-author(s):\n{}", coauthors.join("\n"));
            }
            None => {
                let coauthors = coauthor_repo.get_all();
                let result = MultiSelect::new("Select active co-author(s):", coauthors).prompt();

                match result {
                    Ok(selected) => {
                        coauthor_repo.deactivate_all();

                        selected.clone().into_iter().for_each(|coauthor| {
                            coauthor_repo.activate(&coauthor);
                        });

                        if selected.is_empty() {
                            println!("Going solo!")
                        }
                    }
                    Err(_) => println!("failed to select co-author(s)"),
                }
            }
        },
    }
}

use std::process::Command;
use std::str;

use clap::{Parser, Subcommand};

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

    match &cli.command {
        Commands::With { coauthor_keys } => {
            Command::new("git")
                .arg("config")
                .arg("--global")
                .arg("--remove-section")
                .arg(format!("co-authors.active"))
                .output()
                .expect("failed to execute process");

            match coauthor_keys {
                Some(keys) => {
                    for key in keys {
                        let output = Command::new("git")
                            .arg("config")
                            .arg("--global")
                            .arg(format!("chums.{key}"))
                            .output()
                            .expect("failed to execute process");

                        assert!(output.status.success());

                        let status = Command::new("git")
                            .arg("config")
                            .arg("--global")
                            .arg(format!("chums.active.{key}"))
                            .arg(str::from_utf8(&output.stdout).unwrap().trim())
                            .status()
                            .expect("failed to execute process");

                        assert!(status.success());
                    }
                    return;
                }
                None => println!("No co-author keys provided"),
            }
        }
    }
}

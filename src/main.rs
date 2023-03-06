use std::process::Command;
use std::str;

use clap::{Parser, Subcommand};
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

    match &cli.command {
        Commands::With { coauthor_keys } => {
            Command::new("git")
                .arg("config")
                .arg("--global")
                .arg("--remove-section")
                .arg(format!("coauthors-active"))
                .output()
                .expect("failed to execute process");

            match coauthor_keys {
                Some(keys) => {
                    let coauthors = keys
                        .into_iter()
                        .map(|key| {
                            let output = Command::new("git")
                                .arg("config")
                                .arg("--global")
                                .arg(format!("coauthors.{key}"))
                                .output()
                                .expect("failed to execute process");

                            assert!(output.status.success());
                            let coauthor =
                                String::from_utf8(output.stdout).unwrap().trim().to_string();

                            let status = Command::new("git")
                                .arg("config")
                                .arg("--global")
                                .arg("--add")
                                .arg("coauthors-active.entry")
                                .arg(&coauthor)
                                .status()
                                .expect("failed to execute process");

                            assert!(status.success());
                            return coauthor;
                        })
                        .collect::<Vec<String>>();

                    println!("Active co-author(s):\n{}", coauthors.join("\n"));
                }
                None => {
                    println!("No co-author keys provided");

                    let output = Command::new("git")
                        .arg("config")
                        .arg("--global")
                        .arg("--get-regexp")
                        .arg("^coauthors\\.")
                        .output()
                        .expect("failed to execute process");

                    assert!(output.status.success());
                    let options: Vec<&str> = str::from_utf8(&output.stdout)
                        .unwrap()
                        .lines()
                        .map(|x| x.split_once(' ').unwrap().1)
                        .collect();

                    let result = MultiSelect::new("Select active co-author(s):", options).prompt();

                    match result {
                        Ok(selected) => {
                            selected.clone().into_iter().for_each(|coauthor| {
                                let status = Command::new("git")
                                    .arg("config")
                                    .arg("--global")
                                    .arg("--add")
                                    .arg("coauthors-active.entry")
                                    .arg(coauthor)
                                    .status()
                                    .expect("failed to execute process");

                                assert!(status.success());
                            });
                            if selected.is_empty() {
                                println!("Going solo!")
                            }
                        }
                        Err(_) => println!("failed to select co-author(s)"),
                    }
                }
            }
        }
    }
}

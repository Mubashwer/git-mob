use git_mob_tool::{cli, coauthor_repo::GitConfigCoauthorRepo};
use std::{
    error::Error,
    io::{stderr, stdout},
};

fn main() -> Result<(), Box<dyn Error>> {
    let coauthor_repo = GitConfigCoauthorRepo {};
    let out = &mut stdout();
    let err = &mut stderr();
    cli::run(&coauthor_repo, out, err)?;
    Ok(())
}

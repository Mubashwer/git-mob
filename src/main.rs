use git_mob_tool::{cli, coauthor_repo::GitConfigCoauthorRepo};
use std::{error::Error, io::stdout};

fn main() -> Result<(), Box<dyn Error>> {
    let coauthor_repo = GitConfigCoauthorRepo {};
    let out = &mut stdout();
    cli::run(&coauthor_repo, out)?;
    Ok(())
}

use git_mob_tool::{cli, coauthor_repo::GitConfigCoauthorRepo, helpers::StdCommandRunner};
use std::{error::Error, io::stdout};

fn main() -> Result<(), Box<dyn Error>> {
    let command_runner = StdCommandRunner;
    let coauthor_repo = GitConfigCoauthorRepo { command_runner };
    let out = &mut stdout();
    cli::run(&coauthor_repo, out)?;
    Ok(())
}

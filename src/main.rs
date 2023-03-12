use git_mob::{cli, coauthor_repo::GitConfigCoauthorRepo};
use std::io;

fn main() {
    let coauthor_repo = GitConfigCoauthorRepo {};
    let writer = &mut io::stdout();
    cli::run(&coauthor_repo, writer);
}

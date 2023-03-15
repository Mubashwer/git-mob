use git_mob::{cli, coauthor_repo::GitConfigCoauthorRepo};
use std::io;

fn main() {
    let coauthor_repo = GitConfigCoauthorRepo {};
    let out = &mut io::stdout();
    let err = &mut io::stderr();
    cli::run(&coauthor_repo, out, err);
}

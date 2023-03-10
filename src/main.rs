use git_mob::{cli, coauthor_repo::GitConfigCoauthorRepo};

fn main() {
    let coauthor_repo = GitConfigCoauthorRepo {};
    cli::run(&coauthor_repo);
}

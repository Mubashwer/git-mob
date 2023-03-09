use git_mob::{
    cli,
    coauthor_repo::{CoauthorRepo, GitConfigCoauthorRepo},
};

fn main() {
    let coauthor_repo: Box<dyn CoauthorRepo> = Box::new(GitConfigCoauthorRepo {});
    cli::run(&*coauthor_repo);
}

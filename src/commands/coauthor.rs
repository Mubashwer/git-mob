use clap::{arg, Parser};

use crate::coauthor_repo::CoauthorRepo;

#[derive(Parser)]
pub struct Coauthor {
    /// Adds co-author to co-author repository
    ///
    /// Usage example: git mob co-author --add lm "Leo Messi" leo.messi@example.com
    #[arg(short = 'a', long = "add", num_args=3, value_names=["COAUTHOR_KEY", "COAUTHOR_NAME", "COAUTHOR_EMAIL"])]
    add: Vec<String>,
    /// Remove co-author from co-author repository
    ///
    /// Usage example: git mob co-author --delete lm
    #[arg(short = 'd', long = "delete", value_name = "COAUTHOR_KEY")]
    delete: Option<String>,
    /// Lists co-author(s) from co-author repository
    ///
    /// Usage example: git mob co-author --list
    #[arg(short = 'l', long = "list")]
    list: bool,
}

impl Coauthor {
    pub fn handle(&self, coauthor_repo: &impl CoauthorRepo) {
        if self.list {
            print!("{}", coauthor_repo.list().join("\n"));
        }
    }
}

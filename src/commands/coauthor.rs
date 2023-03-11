use clap::{arg, Parser};

use crate::coauthor_repo::CoauthorRepo;

#[derive(Parser)]
pub struct Coauthor {
    /// Adds co-author to co-author repository
    ///
    /// Usage example: git mob co-author --add lm "Leo Messi" leo.messi@example.com
    #[arg(short = 'a', long = "add", num_args=3, value_names=["COAUTHOR_KEY", "COAUTHOR_NAME", "COAUTHOR_EMAIL"])]
    add: Option<Vec<String>>,
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
        if self.delete.is_some() {
            coauthor_repo.remove(self.delete.as_ref().unwrap());
        }
        if self.list {
            print!("{}", coauthor_repo.list().join("\n"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;

    #[test]
    fn test_delete_removes_coauthor() {
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo.expect_remove().once().return_const({});

        let coauthor = Coauthor {
            delete: Some("lm".to_owned()),
            add: None,
            list: false,
        };

        coauthor.handle(&mock_coauthor_repo);
    }
}

use clap::{arg, Parser};

use crate::coauthor_repo::CoauthorRepo;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct Coauthor {
    /// Adds co-author to co-author repository
    ///
    /// Usage example: git mob co-author --add lm "Leo Messi" leo.messi@example.com
    #[arg(short = 'a', long = "add", num_args=3, value_names=["COAUTHOR_KEY", "COAUTHOR_NAME", "COAUTHOR_EMAIL"])]
    pub(crate) add: Option<Vec<String>>,
    /// Remove co-author from co-author repository
    ///
    /// Usage example: git mob co-author --delete lm
    #[arg(short = 'd', long = "delete", value_name = "COAUTHOR_KEY")]
    pub(crate) delete: Option<String>,
    /// Lists co-author(s) from co-author repository
    ///
    /// Usage example: git mob co-author --list
    #[arg(short = 'l', long = "list")]
    pub(crate) list: bool,
}

impl Coauthor {
    pub(crate) fn handle(&self, coauthor_repo: &impl CoauthorRepo) {
        if self.delete.is_some() {
            coauthor_repo.remove(self.delete.as_ref().unwrap());
        }
        if self.list {
            println!("{}", coauthor_repo.list().join("\n"));
        }
        if self.add.is_some() {
            let coauthor_info = self.add.as_ref().unwrap();
            let key: &str = coauthor_info[0].as_ref();
            let name: &str = coauthor_info[1].as_ref();
            let email: &str = coauthor_info[2].as_ref();

            let coauthor = format!("{name} <{email}>");
            coauthor_repo.add(key, &coauthor);

            println!("{coauthor}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;
    use mockall::predicate;

    #[test]
    fn test_delete_removes_coauthor() {
        let key = "lm";
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_remove()
            .with(predicate::eq(key))
            .once()
            .return_const({});

        let coauthor = Coauthor {
            delete: Some(key.to_owned()),
            add: None,
            list: false,
        };

        coauthor.handle(&mock_coauthor_repo);
    }

    #[test]
    fn test_add_adds_coauthor() {
        let key = "lm";
        let name = "Leo Messi";
        let email = "leo.messi@example.com";

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_add()
            .with(
                predicate::eq(key),
                predicate::eq(format!("{name} <{email}>")),
            )
            .once()
            .return_const({});

        let coauthor = Coauthor {
            add: Some(vec![key.to_owned(), name.to_owned(), email.to_owned()]),
            delete: None,
            list: false,
        };

        coauthor.handle(&mock_coauthor_repo);
    }

    #[test]
    fn test_list_fetches_all_coauthors() {
        let coauthors = vec![
            "Leo Messi <leo.messi@example.com>".to_owned(),
            "Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list()
            .once()
            .return_const(coauthors);

        let mob = Coauthor {
            list: true,
            delete: None,
            add: None,
        };

        mob.handle(&mock_coauthor_repo);
    }
}

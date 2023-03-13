use crate::coauthor_repo::CoauthorRepo;
use clap::{arg, Parser};
use std::io;

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
    pub(crate) fn handle(&self, coauthor_repo: &impl CoauthorRepo, writer: &mut impl io::Write) {
        if self.delete.is_some() {
            let key = self.delete.as_ref().unwrap();
            match coauthor_repo.get(key) {
                Some(_) => coauthor_repo.remove(key),
                None => eprintln!("No co-author found with key: {key}"),
            }
        }
        if self.list {
            writeln!(writer, "{}", coauthor_repo.list().join("\n")).expect("write failed");
        }
        if self.add.is_some() {
            let coauthor_info = self.add.as_ref().unwrap();
            let key: &str = coauthor_info[0].as_ref();
            let name: &str = coauthor_info[1].as_ref();
            let email: &str = coauthor_info[2].as_ref();
            let coauthor = format!("{name} <{email}>");

            coauthor_repo.add(key, &coauthor);
            writeln!(writer, "{coauthor}").expect("write failed");
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
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .return_const("Leo Messi <leo.messi@example.com>".to_owned());
        mock_coauthor_repo
            .expect_remove()
            .with(predicate::eq(key))
            .once()
            .return_const(());

        let coauthor = Coauthor {
            delete: Some(key.to_owned()),
            add: None,
            list: false,
        };

        let mut result = Vec::new();
        coauthor.handle(&mock_coauthor_repo, &mut result);
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
            .return_const(());

        let coauthor_cmd = Coauthor {
            add: Some(vec![key.to_owned(), name.to_owned(), email.to_owned()]),
            delete: None,
            list: false,
        };

        let mut result = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut result);

        assert_eq!(result, format!("{name} <{email}>\n").as_bytes());
    }

    #[test]
    fn test_list_writes_all_coauthors() {
        let coauthors = vec![
            "Leo Messi <leo.messi@example.com>".to_owned(),
            "Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list()
            .once()
            .return_const(coauthors.to_owned());

        let coauthor_cmd = Coauthor {
            list: true,
            delete: None,
            add: None,
        };

        let mut result = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut result);

        assert_eq!(result, format!("{}\n", coauthors.join("\n")).as_bytes());
    }
}

use crate::coauthor_repo::CoauthorRepo;
use clap::{arg, Parser};
use std::io::Write;

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
    /// Lists co-author(s) with keys(s) from co-author repository
    ///
    /// Usage example: git mob co-author --list
    #[arg(short = 'l', long = "list")]
    pub(crate) list: bool,
}

impl Coauthor {
    pub(crate) fn handle(
        &self,
        coauthor_repo: &impl CoauthorRepo,
        out: &mut impl Write,
        err: &mut impl Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(key) = self.delete.as_deref() {
            match coauthor_repo.get(key)? {
                Some(_) => coauthor_repo.remove(key)?,
                None => writeln!(err, "No co-author found with key: {key}")?,
            }
        }
        if self.list {
            let coauthors = coauthor_repo.list(true)?;
            if !coauthors.is_empty() {
                writeln!(out, "{}", coauthors.join("\n"))?
            }
        }
        if let Some([key, name, email]) = self.add.as_deref() {
            let coauthor = format!("{name} <{email}>");
            coauthor_repo.add(key, &coauthor)?;
            writeln!(out, "{coauthor}")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;
    use mockall::predicate;

    #[test]
    fn test_delete_coauthor() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(Some("Leo Messi <leo.messi@example.com>".to_owned())));
        mock_coauthor_repo
            .expect_remove()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(()));

        let coauthor_cmd = Coauthor {
            delete: Some(key.to_owned()),
            add: None,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        Ok(())
    }

    #[test]
    fn test_error_message_shown_when_trying_to_delete_coauthor_with_non_existing_coauthor_key(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = "em";
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(None));

        let coauthor_cmd = Coauthor {
            delete: Some(key.to_owned()),
            add: None,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(
            err,
            format!("No co-author found with key: {key}\n").as_bytes()
        );

        Ok(())
    }

    #[test]
    fn test_add_coauthor() -> Result<(), Box<dyn std::error::Error>> {
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
            .returning(|_, _| Ok(()));

        let coauthor_cmd = Coauthor {
            add: Some(vec![key.to_owned(), name.to_owned(), email.to_owned()]),
            delete: None,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(out, format!("{name} <{email}>\n").as_bytes());

        Ok(())
    }

    #[test]
    fn test_list_coauthors() -> Result<(), Box<dyn std::error::Error>> {
        let coauthors = vec![
            "lm Leo Messi <leo.messi@example.com>".to_owned(),
            "em Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let expected_output = format!("{}\n", coauthors.join("\n"));

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list()
            .once()
            .returning(move |_| Ok(coauthors.to_owned()));

        let coauthor_cmd = Coauthor {
            list: true,
            delete: None,
            add: None,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(out, expected_output.as_bytes());

        Ok(())
    }
}

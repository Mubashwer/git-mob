use crate::coauthor_repo::CoauthorRepo;
use clap::{arg, Parser};
use inquire::MultiSelect;
use std::{error::Error, io::Write};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct Mob {
    /// Sets active co-author(s) for pair/mob programming session
    ///
    /// Usage example: git mob pair --with lm mj
    #[arg(short='w', long="with", num_args=0.., value_name="COAUTHOR_KEY")]
    pub(crate) with: Option<Vec<String>>,
    /// Clears mob/pair programming session. Going solo!
    ///
    /// Usage example: git mob co-author --list
    #[arg(short = 'c', long = "clear")]
    pub(crate) clear: bool,
    /// Lists co-author(s) in current mob/pair programming session
    ///
    /// Usage example: git mob --list
    #[arg(short = 'l', long = "list")]
    pub(crate) list: bool,
    /// Lists Co-authored-by trailers in current mob/pair programming session
    ///
    /// Usage example: git mob --trailers
    #[arg(short = 't', long = "trailers")]
    pub(crate) trailers: bool,
}

impl Mob {
    pub(crate) fn handle(
        &self,
        coauthor_repo: &impl CoauthorRepo,
        out: &mut impl Write,
        err: &mut impl Write,
    ) -> Result<(), Box<dyn Error>> {
        if self.clear {
            coauthor_repo.clear_mob()?;
        }

        if self.list {
            let coauthors = coauthor_repo.list_mob()?;
            if !coauthors.is_empty() {
                writeln!(out, "{}", coauthors.join("\n"))?
            }
        }

        if self.trailers {
            let coauthors = coauthor_repo.list_mob()?;
            let trailers = coauthors
                .iter()
                .map(|x| format!("Co-authored-by: {x}"))
                .collect::<Vec<String>>()
                .join("\n");

            if !coauthors.is_empty() {
                writeln!(out, "{}", trailers)?
            }
        }

        match self.with.as_deref() {
            None => {}
            Some([]) => {
                let coauthors = coauthor_repo.list(false)?;
                if coauthors.is_empty() {
                    writeln!(
                        err,
                        "No co-author(s) found. At least one co-author must be added"
                    )?;
                    return Ok(());
                }

                let result = MultiSelect::new("Select active co-author(s):", coauthors).prompt();
                match &result {
                    Ok(selected) => {
                        coauthor_repo.clear_mob()?;
                        for coauthor in selected.iter() {
                            coauthor_repo.add_to_mob(coauthor)?;
                        }

                        if selected.is_empty() {
                            writeln!(out, "Going solo!")?
                        }
                    }
                    Err(_) => writeln!(err, "Failed to prompt selection of co-author(s)")?,
                }
            }
            Some(coauthor_keys) => {
                let mut coauthors: Vec<String> = Vec::new();
                coauthor_repo.clear_mob()?;

                for key in coauthor_keys {
                    match coauthor_repo.get(key)? {
                        Some(coauthor) => {
                            coauthor_repo.add_to_mob(&coauthor)?;
                            coauthors.push(coauthor);
                        }
                        None => writeln!(err, "No co-author found with key: {key}")?,
                    }
                }

                if !coauthors.is_empty() {
                    writeln!(out, "{}", coauthors.join("\n"))?
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;
    use mockall::predicate;

    #[test]
    fn test_clear_mob_session() -> Result<(), Box<dyn Error>> {
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_clear_mob()
            .once()
            .returning(|| Ok(()));

        let mob_cmd = Mob {
            clear: true,
            with: None,
            list: false,
            trailers: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        Ok(())
    }

    #[test]
    fn test_list_mob_coauthors() -> Result<(), Box<dyn Error>> {
        let coauthors = vec![
            "Leo Messi <leo.messi@example.com>".to_owned(),
            "Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let expected_output = format!("{}\n", coauthors.join("\n"));

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list_mob()
            .once()
            .returning(move || Ok(coauthors.to_owned()));

        let mob_cmd = Mob {
            list: true,
            clear: false,
            with: None,
            trailers: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(out, expected_output.as_bytes());

        Ok(())
    }

    #[test]
    fn test_list_mob_coauthors_when_mob_session_is_empty() -> Result<(), Box<dyn Error>> {
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list_mob()
            .once()
            .returning(move || Ok(vec![]));

        let mob_cmd = Mob {
            list: true,
            clear: false,
            with: None,
            trailers: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(out, b"");

        Ok(())
    }

    #[test]
    fn test_mob_coauthor_trailers() -> Result<(), Box<dyn Error>> {
        let coauthors = vec![
            "Leo Messi <leo.messi@example.com>".to_owned(),
            "Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list_mob()
            .once()
            .returning(move || Ok(coauthors.to_owned()));

        let mob_cmd = Mob {
            list: false,
            clear: false,
            with: None,
            trailers: true,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(out, b"Co-authored-by: Leo Messi <leo.messi@example.com>\nCo-authored-by: Emi Martinez <emi.martinez@example.com>\n");

        Ok(())
    }

    #[test]
    fn test_mob_coauthor_trailers_when_mob_session_is_empty() -> Result<(), Box<dyn Error>> {
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list_mob()
            .once()
            .returning(move || Ok(vec![]));

        let mob_cmd = Mob {
            list: false,
            clear: false,
            with: None,
            trailers: true,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(out, b"");

        Ok(())
    }

    #[test]
    fn test_error_message_shown_when_trying_to_mob_given_coauthors_list_is_empty(
    ) -> Result<(), Box<dyn Error>> {
        let coauthors = vec![];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list()
            .with(predicate::eq(false))
            .once()
            .returning(move |_| Ok(coauthors.to_owned()));

        let mob_cmd = Mob {
            with: Some(vec![]),
            clear: false,
            list: false,
            trailers: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(
            err,
            b"No co-author(s) found. At least one co-author must be added\n"
        );

        Ok(())
    }

    #[test]
    fn test_adding_coauthors_to_mob_session_by_keys() -> Result<(), Box<dyn Error>> {
        let keys = vec!["lm".to_owned(), "em".to_owned()];
        let coauthors = [
            "Leo Messi <leo.messi@example.com>",
            "Emi Martinez <emi.martinez@example.com>",
        ];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_clear_mob()
            .once()
            .returning(|| Ok(()));
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(keys[0].to_owned()))
            .once()
            .returning(move |_| Ok(Some(coauthors[0].to_owned())));
        mock_coauthor_repo
            .expect_add_to_mob()
            .with(predicate::eq(coauthors[0].to_owned()))
            .once()
            .returning(|_| Ok(()));
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(keys[1].to_owned()))
            .once()
            .returning(move |_| Ok(Some(coauthors[1].to_owned())));
        mock_coauthor_repo
            .expect_add_to_mob()
            .with(predicate::eq(coauthors[1].to_owned()))
            .once()
            .returning(|_| Ok(()));

        let mob_cmd = Mob {
            with: Some(keys),
            clear: false,
            list: false,
            trailers: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(out, format!("{}\n", coauthors.join("\n")).as_bytes());

        Ok(())
    }

    #[test]
    fn test_error_message_shown_when_trying_to_mob_while_passing_non_existing_coauthor_key(
    ) -> Result<(), Box<dyn Error>> {
        let key = "lm";

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_clear_mob()
            .once()
            .returning(|| Ok(()));
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(None));

        let mob_cmd = Mob {
            with: Some(vec![key.to_owned()]),
            clear: false,
            list: false,
            trailers: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err)?;

        assert_eq!(
            err,
            format!("No co-author found with key: {key}\n").as_bytes()
        );

        Ok(())
    }
}

use crate::coauthor_repo::CoauthorRepo;
use clap::{arg, Parser};
use inquire::MultiSelect;
use std::io;

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
}

impl Mob {
    pub(crate) fn handle(
        &self,
        coauthor_repo: &impl CoauthorRepo,
        out: &mut impl io::Write,
        err: &mut impl io::Write,
    ) {
        if self.clear {
            coauthor_repo.clear_mob();
        }
        if self.list {
            let coauthors = coauthor_repo.list_mob();
            if !coauthors.is_empty() {
                writeln!(out, "{}", coauthors.join("\n")).expect("write failed");
            }
        }

        match self.with.as_deref() {
            None => {}
            Some([]) => {
                let coauthors = coauthor_repo.list(false);
                if coauthors.is_empty() {
                    writeln!(
                        err,
                        "No co-author(s) found. At least one co-author must be added"
                    )
                    .expect("write failed");
                    return;
                }

                let result = MultiSelect::new("Select active co-author(s):", coauthors).prompt();
                match &result {
                    Ok(selected) => {
                        coauthor_repo.clear_mob();
                        selected.iter().for_each(|coauthor| {
                            coauthor_repo.add_to_mob(coauthor);
                        });

                        if selected.is_empty() {
                            writeln!(out, "Going solo!").expect("write failed");
                        }
                    }
                    Err(_) => writeln!(err, "Failed to prompt selection of co-author(s)")
                        .expect("write failed"),
                }
            }
            Some(coauthor_keys) => {
                let mut coauthors: Vec<String> = Vec::new();
                coauthor_repo.clear_mob();

                for key in coauthor_keys {
                    match coauthor_repo.get(key) {
                        Some(coauthor) => {
                            coauthor_repo.add_to_mob(&coauthor);
                            coauthors.push(coauthor);
                        }
                        None => writeln!(err, "No co-author found with key: {key}")
                            .expect("write failed"),
                    }
                }

                if !coauthors.is_empty() {
                    writeln!(out, "{}", coauthors.join("\n")).expect("write failed");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;
    use mockall::predicate;

    #[test]
    fn test_clear_mob_session() {
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_clear_mob()
            .once()
            .return_const(());

        let mob_cmd = Mob {
            clear: true,
            with: None,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);
    }

    #[test]
    fn test_list_mob_coauthors() {
        let coauthors = [
            "Leo Messi <leo.messi@example.com>".to_owned(),
            "Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list_mob()
            .once()
            .return_const(coauthors.to_owned());

        let mob_cmd = Mob {
            list: true,
            clear: false,
            with: None,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);

        assert_eq!(out, format!("{}\n", coauthors.join("\n")).as_bytes());
    }

    #[test]
    fn test_error_message_shown_when_trying_to_mob_given_coauthors_list_is_empty() {
        let coauthors = [];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list()
            .with(predicate::eq(false))
            .once()
            .return_const(coauthors.to_owned());

        let mob_cmd = Mob {
            with: Some(vec![]),
            clear: false,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);

        assert_eq!(
            err,
            b"No co-author(s) found. At least one co-author must be added\n"
        );
    }

    #[test]
    fn test_adding_coauthors_to_mob_session_by_keys() {
        let keys = vec!["lm".to_owned(), "em".to_owned()];
        let coauthors = [
            "Leo Messi <leo.messi@example.com>",
            "Emi Martinez <emi.martinez@example.com>",
        ];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_clear_mob()
            .once()
            .return_const(());
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(keys[0].to_owned()))
            .once()
            .return_const(coauthors[0].to_owned());
        mock_coauthor_repo
            .expect_add_to_mob()
            .with(predicate::eq(coauthors[0].to_owned()))
            .once()
            .return_const(());
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(keys[1].to_owned()))
            .once()
            .return_const(coauthors[1].to_owned());
        mock_coauthor_repo
            .expect_add_to_mob()
            .with(predicate::eq(coauthors[1].to_owned()))
            .once()
            .return_const(());

        let mob_cmd = Mob {
            with: Some(keys),
            clear: false,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);

        assert_eq!(out, format!("{}\n", coauthors.join("\n")).as_bytes());
    }

    #[test]
    fn test_error_message_shown_when_trying_to_mob_while_passing_non_existing_coauthor_key() {
        let key = "lm";

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_clear_mob()
            .once()
            .return_const(());
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .return_const(None);

        let mob_cmd = Mob {
            with: Some(vec![key.to_owned()]),
            clear: false,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);

        assert_eq!(
            err,
            format!("No co-author found with key: {key}\n").as_bytes()
        );
    }
}

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
        if self.clear || self.with.is_some() {
            coauthor_repo.clear_mob();
        }
        if self.list {
            writeln!(out, "{}", coauthor_repo.list_mob().join("\n")).expect("write failed");
        }
        if self.with.is_none() {
            return;
        }

        let coauthor_keys = self.with.as_ref().unwrap();

        match coauthor_keys.is_empty() {
            true => {
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

                match result {
                    Ok(selected) => {
                        selected.clone().into_iter().for_each(|coauthor| {
                            coauthor_repo.add_to_mob(&coauthor);
                        });

                        if selected.is_empty() {
                            writeln!(out, "Going solo!").expect("write failed");
                        }
                    }
                    Err(_) => writeln!(err, "Failed to prompt selection of co-author(s)")
                        .expect("write failed"),
                }
            }
            false => {
                let mut coauthors: Vec<String> = Vec::new();
                for key in coauthor_keys {
                    match coauthor_repo.get(&key) {
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

    #[test]
    fn test_clear_clears_mob() {
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
    fn test_list_writes_mob_coauthors() {
        let coauthors = vec![
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
}

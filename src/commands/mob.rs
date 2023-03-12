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
    pub(crate) fn handle(&self, coauthor_repo: &impl CoauthorRepo, writer: &mut impl io::Write) {
        if self.clear || self.with.is_some() {
            coauthor_repo.clear_mob();
        }
        if self.list {
            writeln!(writer, "{}", coauthor_repo.list_mob().join("\n")).expect("write failed");
        }
        if self.with.is_none() {
            return;
        }

        let coauthor_keys = self.with.as_ref().unwrap();

        match coauthor_keys.len() {
            0 => {
                let coauthors = coauthor_repo.list();
                let result = MultiSelect::new("Select active co-author(s):", coauthors).prompt();

                match result {
                    Ok(selected) => {
                        selected.clone().into_iter().for_each(|coauthor| {
                            coauthor_repo.add_to_mob(&coauthor);
                        });

                        if selected.is_empty() {
                            writeln!(writer, "Going solo!").expect("write failed");
                        }
                    }
                    Err(_) => eprintln!("failed to select co-author(s)"),
                }
            }
            _ => {
                let coauthors: Vec<String> = coauthor_keys
                    .into_iter()
                    .map(|key| {
                        let coauthor = coauthor_repo.get(&key);
                        coauthor_repo.add_to_mob(&coauthor);
                        return coauthor;
                    })
                    .collect();

                writeln!(writer, "{}", coauthors.join("\n")).expect("write failed");
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
            .return_const({});

        let mob_cmd = Mob {
            clear: true,
            with: None,
            list: false,
        };

        let mut result = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut result);
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

        let mut result = Vec::new();
        mob_cmd.handle(&mock_coauthor_repo, &mut result);

        assert_eq!(result, format!("{}\n", coauthors.join("\n")).as_bytes());
    }
}

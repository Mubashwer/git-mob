use clap::{arg, Parser};
use inquire::MultiSelect;

use crate::coauthor_repo::CoauthorRepo;

#[derive(Parser)]
pub struct Mob {
    /// Sets active co-author(s) for pair/mob programming
    #[arg(short='w', long="with", num_args=0.., value_name="COAUTHOR_KEY")]
    with: Option<Vec<String>>,
    /// Clears mob/pair programming session. Going solo!
    #[arg(short = 'c', long = "clear")]
    clear: bool,
}

impl Mob {
    pub fn handle(&self, coauthor_repo: &dyn CoauthorRepo) {
        if self.clear {
            coauthor_repo.deactivate_all();
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
                        coauthor_repo.deactivate_all();

                        selected.clone().into_iter().for_each(|coauthor| {
                            coauthor_repo.activate(&coauthor);
                        });

                        if selected.is_empty() {
                            println!("Going solo!")
                        }
                    }
                    Err(_) => println!("failed to select co-author(s)"),
                }
            }
            _ => {
                coauthor_repo.deactivate_all();

                let coauthors: Vec<String> = coauthor_keys
                    .into_iter()
                    .map(|key| {
                        let coauthor = coauthor_repo.get(&key);
                        coauthor_repo.activate(&coauthor);
                        return coauthor;
                    })
                    .collect();

                println!("{}", coauthors.join("\n"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;

    #[test]
    fn test_clear_deactivates_all_coauthors() {
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_deactivate_all()
            .once()
            .return_const({});

        let mob = Mob {
            with: None,
            clear: true,
        };

        mob.handle(&mock_coauthor_repo);
    }
}

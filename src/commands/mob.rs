use crate::repositories::{MobSessionRepo, TeamMemberRepo};
use crate::Result;
use clap::{arg, Parser};
use inquire::MultiSelect;
use std::io::Write;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct Mob {
    /// Sets co-author(s) from team member(s) in the mob/pair programming session
    ///
    /// This will clear any existing co-author(s) in current session
    ///
    /// Usage example: git mob pair --with lm mj
    #[arg(short='w', long="with", num_args=0.., value_name="COAUTHOR_KEY")]
    pub(crate) with: Option<Vec<String>>,
    /// Adds co-author to the mob/pair programming session (usually non-team member)
    ///
    /// Usage example: git mob --add "Leo Messi" leo.messi@example.com
    #[arg(short = 'a', long = "add", num_args=2, value_names=["COAUTHOR_NAME", "COAUTHOR_EMAIL"])]
    pub(crate) add: Option<Vec<String>>,
    /// Clears the mob/pair programming session. Going solo!
    ///
    /// Usage example: git mob --clear
    #[arg(short = 'c', long = "clear")]
    pub(crate) clear: bool,
    /// Lists co-author(s) in the mob/pair programming session
    ///
    /// Usage example: git mob --list
    #[arg(short = 'l', long = "list")]
    pub(crate) list: bool,
    /// Lists Co-authored-by trailers in the mob/pair programming session
    ///
    /// Usage example: git mob --trailers
    #[arg(short = 't', long = "trailers")]
    pub(crate) trailers: bool,
}

impl Mob {
    pub(crate) fn handle(
        &self,
        team_member_repo: &impl TeamMemberRepo,
        mob_repo: &impl MobSessionRepo,
        out: &mut impl Write,
    ) -> Result<()> {
        if self.clear {
            mob_repo.clear()?;
        }

        if self.list {
            let coauthors = mob_repo.list_coauthors()?;
            if !coauthors.is_empty() {
                writeln!(out, "{}", coauthors.join("\n"))?
            }
        }

        if self.trailers {
            let coauthors = mob_repo.list_coauthors()?;
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
                let team_members = team_member_repo.list(false)?;
                if team_members.is_empty() {
                    return Err(
                        "No team member(s) found. At least one team member must be added".into(),
                    );
                }

                let result = MultiSelect::new("Select active co-author(s):", team_members)
                    .prompt_skippable()?;
                if let Some(selected) = result {
                    mob_repo.clear()?;
                    for team_member in selected.iter() {
                        mob_repo.add_coauthor(team_member)?;
                    }

                    if selected.is_empty() {
                        writeln!(out, "Going solo!")?
                    }
                }
            }
            Some(team_member_keys) => {
                let mut coauthors: Vec<String> = Vec::new();
                mob_repo.clear()?;

                for key in team_member_keys {
                    match team_member_repo.get(key)? {
                        Some(team_member) => {
                            mob_repo.add_coauthor(&team_member)?;
                            coauthors.push(team_member);
                        }
                        None => return Err(format!("No team member found with key: {key}").into()),
                    }
                }

                writeln!(out, "{}", coauthors.join("\n"))?
            }
        }

        if let Some([name, email]) = self.add.as_deref() {
            let coauthor = format!("{name} <{email}>");
            mob_repo.add_coauthor(&coauthor)?;
            writeln!(out, "{coauthor}")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{MockMobSessionRepo, MockTeamMemberRepo};
    use mockall::predicate;

    #[test]
    fn test_clear_mob() -> Result<()> {
        let mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo.expect_clear().once().returning(|| Ok(()));

        let mob_cmd = Mob {
            clear: true,
            with: None,
            list: false,
            trailers: false,
            add: None,
        };

        let mut out = Vec::new();
        mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out)?;

        Ok(())
    }

    #[test]
    fn test_list_mob() -> Result<()> {
        let coauthors = vec![
            "Leo Messi <leo.messi@example.com>".to_owned(),
            "Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let expected_output = format!("{}\n", coauthors.join("\n"));

        let mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo
            .expect_list_coauthors()
            .once()
            .returning(move || Ok(coauthors.to_owned()));

        let mob_cmd = Mob {
            list: true,
            clear: false,
            with: None,
            trailers: false,
            add: None,
        };

        let mut out = Vec::new();
        mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out)?;

        assert_eq!(out, expected_output.as_bytes());

        Ok(())
    }

    #[test]
    fn test_list_mob_when_mob_session_is_empty() -> Result<()> {
        let mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo
            .expect_list_coauthors()
            .once()
            .returning(move || Ok(vec![]));

        let mob_cmd = Mob {
            list: true,
            clear: false,
            with: None,
            trailers: false,
            add: None,
        };

        let mut out = Vec::new();
        mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out)?;

        assert_eq!(out, b"");

        Ok(())
    }

    #[test]
    fn test_mob_coauthor_trailers() -> Result<()> {
        let coauthors = vec![
            "Leo Messi <leo.messi@example.com>".to_owned(),
            "Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo
            .expect_list_coauthors()
            .once()
            .returning(move || Ok(coauthors.to_owned()));

        let mob_cmd = Mob {
            list: false,
            clear: false,
            with: None,
            trailers: true,
            add: None,
        };

        let mut out = Vec::new();
        mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out)?;

        assert_eq!(
            out,
            b"Co-authored-by: Leo Messi <leo.messi@example.com>\n\
              Co-authored-by: Emi Martinez <emi.martinez@example.com>\n"
        );

        Ok(())
    }

    #[test]
    fn test_mob_coauthor_trailers_when_mob_session_is_empty() -> Result<()> {
        let mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo
            .expect_list_coauthors()
            .once()
            .returning(move || Ok(vec![]));

        let mob_cmd = Mob {
            list: false,
            clear: false,
            with: None,
            trailers: true,
            add: None,
        };

        let mut out = Vec::new();
        mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out)?;

        assert_eq!(out, b"");

        Ok(())
    }

    #[test]
    fn test_mob_with_given_no_team_members_added() -> Result<()> {
        let coauthors = vec![];

        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        let mock_mob_repo = MockMobSessionRepo::new();
        mock_team_member_repo
            .expect_list()
            .with(predicate::eq(false))
            .once()
            .returning(move |_| Ok(coauthors.to_owned()));

        let mob_cmd = Mob {
            with: Some(vec![]),
            clear: false,
            list: false,
            trailers: false,
            add: None,
        };

        let mut out = Vec::new();
        let result = mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out);

        assert!(result.is_err_and(|err| err.to_string()
            == *"No team member(s) found. At least one team member must be added"));

        Ok(())
    }

    #[test]
    fn test_mob_with_by_keys() -> Result<()> {
        let keys = vec!["lm".to_owned(), "em".to_owned()];
        let coauthors = [
            "Leo Messi <leo.messi@example.com>",
            "Emi Martinez <emi.martinez@example.com>",
        ];

        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo.expect_clear().once().returning(|| Ok(()));
        mock_team_member_repo
            .expect_get()
            .with(predicate::eq(keys[0].to_owned()))
            .once()
            .returning(move |_| Ok(Some(coauthors[0].to_owned())));
        mock_mob_repo
            .expect_add_coauthor()
            .with(predicate::eq(coauthors[0].to_owned()))
            .once()
            .returning(|_| Ok(()));
        mock_team_member_repo
            .expect_get()
            .with(predicate::eq(keys[1].to_owned()))
            .once()
            .returning(move |_| Ok(Some(coauthors[1].to_owned())));
        mock_mob_repo
            .expect_add_coauthor()
            .with(predicate::eq(coauthors[1].to_owned()))
            .once()
            .returning(|_| Ok(()));

        let mob_cmd = Mob {
            with: Some(keys),
            clear: false,
            list: false,
            trailers: false,
            add: None,
        };

        let mut out = Vec::new();
        mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out)?;

        assert_eq!(out, format!("{}\n", coauthors.join("\n")).as_bytes());

        Ok(())
    }

    #[test]
    fn test_mob_with_by_key_when_team_member_not_found() -> Result<()> {
        let key = "lm";

        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo.expect_clear().once().returning(|| Ok(()));
        mock_team_member_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(None));

        let mob_cmd = Mob {
            with: Some(vec![key.to_owned()]),
            clear: false,
            list: false,
            trailers: false,
            add: None,
        };

        let mut out = Vec::new();
        let result = mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out);

        assert!(result
            .is_err_and(|err| err.to_string() == format!("No team member found with key: {key}")));

        Ok(())
    }

    #[test]
    fn test_add_coauthor() -> Result<()> {
        let name = "Leo Messi";
        let email = "leo.messi@example.com";

        let mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo
            .expect_add_coauthor()
            .with(predicate::eq(format!("{name} <{email}>")))
            .once()
            .returning(|_| Ok(()));

        let mob_cmd = Mob {
            add: Some(vec![name.to_owned(), email.to_owned()]),
            with: None,
            clear: false,
            list: false,
            trailers: false,
        };

        let mut out = Vec::new();
        mob_cmd.handle(&mock_team_member_repo, &mock_mob_repo, &mut out)?;

        assert_eq!(out, format!("{name} <{email}>\n").as_bytes());

        Ok(())
    }
}

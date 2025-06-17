use crate::repositories::TeamMemberRepo;
use clap::{arg, Parser};
use std::io::Write;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct TeamMember {
    /// Adds team member to team member repository
    ///
    /// Usage example: git mob team-member --add lm "Leo Messi" leo.messi@example.com
    #[arg(short = 'a', long = "add", num_args=3, value_names=["TEAM_MEMBER_KEY", "TEAM_MEMBER_NAME", "TEAM_MEMBER_EMAIL"])]
    pub(crate) add: Option<Vec<String>>,
    /// Remove team member from team member repository
    ///
    /// Usage example: git mob team-member --delete lm
    #[arg(short = 'd', long = "delete", value_name = "TEAM_MEMBER_KEY")]
    pub(crate) delete: Option<String>,
    /// Lists team member(s) with keys(s) from team member repository
    ///
    /// Usage example: git mob team-member --list
    #[arg(short = 'l', long = "list")]
    pub(crate) list: bool,
}

impl TeamMember {
    pub(crate) fn handle(
        &self,
        team_member_repo: &impl TeamMemberRepo,
        out: &mut impl Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(key) = self.delete.as_deref() {
            match team_member_repo.get(key)? {
                Some(_) => team_member_repo.remove(key)?,
                None => return Err(format!("No team member found with key: {key}").into()),
            }
        }
        if self.list {
            let team_members = team_member_repo.list(true)?;
            if !team_members.is_empty() {
                writeln!(out, "{}", team_members.join("\n"))?
            }
        }
        if let Some([key, name, email]) = self.add.as_deref() {
            let team_member = format!("{name} <{email}>");
            team_member_repo.add(key, &team_member)?;
            writeln!(out, "{team_member}")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MockTeamMemberRepo;
    use mockall::predicate;

    #[test]
    fn test_delete_team_member() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        mock_team_member_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(Some("Leo Messi <leo.messi@example.com>".to_owned())));
        mock_team_member_repo
            .expect_remove()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(()));

        let team_member_cmd = TeamMember {
            delete: Some(key.to_owned()),
            add: None,
            list: false,
        };

        let mut out = Vec::new();
        team_member_cmd.handle(&mock_team_member_repo, &mut out)?;

        assert!(out.is_empty());

        Ok(())
    }

    #[test]
    fn test_delete_team_member_when_team_member_not_found() -> Result<(), Box<dyn std::error::Error>>
    {
        let key = "em";
        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        mock_team_member_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(None));

        let team_member_cmd = TeamMember {
            delete: Some(key.to_owned()),
            add: None,
            list: false,
        };

        let mut out = Vec::new();
        let result = team_member_cmd.handle(&mock_team_member_repo, &mut out);

        assert!(result
            .is_err_and(|err| err.to_string() == format!("No team member found with key: {key}")));

        Ok(())
    }

    #[test]
    fn test_add_team_member() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let name = "Leo Messi";
        let email = "leo.messi@example.com";

        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        mock_team_member_repo
            .expect_add()
            .with(
                predicate::eq(key),
                predicate::eq(format!("{name} <{email}>")),
            )
            .once()
            .returning(|_, _| Ok(()));

        let team_member_cmd = TeamMember {
            add: Some(vec![key.to_owned(), name.to_owned(), email.to_owned()]),
            delete: None,
            list: false,
        };

        let mut out = Vec::new();
        team_member_cmd.handle(&mock_team_member_repo, &mut out)?;

        assert_eq!(out, format!("{name} <{email}>\n").as_bytes());

        Ok(())
    }

    #[test]
    fn test_list_team_members() -> Result<(), Box<dyn std::error::Error>> {
        let team_members = vec![
            "lm Leo Messi <leo.messi@example.com>".to_owned(),
            "em Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let expected_output = format!("{}\n", team_members.join("\n"));

        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        mock_team_member_repo
            .expect_list()
            .once()
            .returning(move |_| Ok(team_members.to_owned()));

        let team_member_cmd = TeamMember {
            list: true,
            delete: None,
            add: None,
        };

        let mut out = Vec::new();
        team_member_cmd.handle(&mock_team_member_repo, &mut out)?;

        assert_eq!(out, expected_output.as_bytes());

        Ok(())
    }

    #[test]
    fn test_list_team_members_when_no_team_members_added() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        mock_team_member_repo
            .expect_list()
            .once()
            .returning(move |_| Ok(vec![]));

        let team_member_cmd = TeamMember {
            list: true,
            delete: None,
            add: None,
        };

        let mut out = Vec::new();
        team_member_cmd.handle(&mock_team_member_repo, &mut out)?;

        assert_eq!(out, b"");

        Ok(())
    }
}

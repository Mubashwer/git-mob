use crate::Result;
use crate::helpers::{CmdOutput, CommandRunner};
use anyhow::{Context, anyhow, bail};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait TeamMemberRepo {
    fn list(&self, show_keys: bool) -> Result<Vec<String>>;
    fn get(&self, key: &str) -> Result<Option<String>>;
    fn remove(&self, key: &str) -> Result<()>;
    fn add(&self, key: &str, team_member: &str) -> Result<()>;
}

pub struct GitConfigTeamMemberRepo<Cmd> {
    pub command_runner: Cmd,
}

impl<Cmd: CommandRunner> GitConfigTeamMemberRepo<Cmd> {
    // Keeping it as coauthors instead of team-members for backwards compatibility
    const COAUTHORS_SECTION: &'static str = "coauthors";

    const EXIT_CODE_SUCCESS: i32 = 0;
    const EXIT_CODE_CONFIG_INVALID_KEY: i32 = 1;

    fn git_config_error<T>(output: &CmdOutput) -> Result<T> {
        match output.status_code {
            Some(code) => bail!("Git config command exited with status code: {code}"),
            None => bail!("Git config command terminated by signal"),
        }
    }
}

impl<Cmd: CommandRunner> TeamMemberRepo for GitConfigTeamMemberRepo<Cmd> {
    fn list(&self, show_keys: bool) -> Result<Vec<String>> {
        let section = Self::COAUTHORS_SECTION;
        let search_regex = format!("^{section}\\.");

        let output = self
            .command_runner
            .execute(
                "git",
                &["config", "--global", "--get-regexp", &search_regex],
            )
            .context("Failed to list team members from git config")?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => String::from_utf8(output.stdout)?
                .lines()
                .map(|x| {
                    let delimiter = if show_keys {
                        format!("{section}.")
                    } else {
                        " ".to_owned()
                    };
                    x.split_once(&delimiter)
                        .ok_or_else(|| anyhow!("Failed to split string: '{x}'"))
                        .map(|(_, team_member)| team_member.to_owned())
                })
                .collect(),

            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Ok(vec![]),
            _ => Self::git_config_error(&output),
        }
    }

    fn get(&self, key: &str) -> Result<Option<String>> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", &full_key])
            .with_context(|| format!("Failed to get team member '{key}' from git config"))?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(Some(
                String::from_utf8(output.stdout)
                    .context("Failed to parse git config output as UTF-8")?
                    .trim()
                    .into(),
            )),
            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Ok(None),
            _ => Self::git_config_error(&output),
        }
    }

    fn remove(&self, key: &str) -> Result<()> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", "--unset-all", &full_key])
            .with_context(|| format!("Failed to remove team member '{key}' from git config"))?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(()),
            _ => Self::git_config_error(&output),
        }
    }

    fn add(&self, key: &str, team_member: &str) -> Result<()> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", &full_key, team_member])
            .with_context(|| format!("Failed to add team member '{key}' to git config"))?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(()),
            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => {
                bail!("Invalid key: {key}")
            }
            _ => Self::git_config_error(&output),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::MockCommandRunner;

    use super::*;

    fn create_mock_command_runner(
        program: &str,
        args: &[&str],
        stdout: Vec<u8>,
        stderr: Vec<u8>,
        status_code: Option<i32>,
    ) -> MockCommandRunner {
        let cloned_program = program.to_string();
        let cloned_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();

        let mut mock_cmd_runner = MockCommandRunner::new();
        mock_cmd_runner
            .expect_execute()
            .once()
            //.with(predicate::eq(cloned_program), predicate::eq(cloned_args)) // this is not working!
            .withf(move |program, args| program == cloned_program && args == cloned_args)
            .returning(move |_, _| {
                Ok(CmdOutput {
                    stdout: stdout.clone(),
                    stderr: stderr.clone(),
                    status_code,
                })
            });
        mock_cmd_runner
    }

    #[test]
    fn test_list_without_keys() -> Result<()> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = b"coauthors.lm Leo Messi <leo.messi@example.com>\ncoauthors.em Emi Martinez <emi.martinez@example.com>\n".into();
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.list(false)?;

        assert_eq!(
            result,
            vec![
                "Leo Messi <leo.messi@example.com>",
                "Emi Martinez <emi.martinez@example.com>"
            ]
        );

        Ok(())
    }

    #[test]
    fn test_list_with_keys() -> Result<()> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = b"coauthors.lm Leo Messi <leo.messi@example.com>\ncoauthors.em Emi Martinez <emi.martinez@example.com>\n".into();
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.list(true)?;

        assert_eq!(
            result,
            vec![
                "lm Leo Messi <leo.messi@example.com>",
                "em Emi Martinez <emi.martinez@example.com>"
            ]
        );

        Ok(())
    }

    #[test]
    fn test_list_when_no_team_members_added() -> Result<()> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.list(true)?;

        assert_eq!(result, Vec::<String>::new());

        Ok(())
    }

    #[test]
    fn test_list_when_unexpected_error() -> Result<()> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.list(true);

        assert!(
            result
                .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129")
        );

        Ok(())
    }

    #[test]
    fn test_list_when_terminated_by_signal() -> Result<()> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.list(true);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_get() -> Result<()> {
        let key = "lm";
        let args = &["config", "--global", &format!("coauthors.{key}")];
        let stdout = b"Leo Messi <leo.messi@example.com>\n".into();
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.get(key)?;

        assert_eq!(result, Some("Leo Messi <leo.messi@example.com>".into()));

        Ok(())
    }

    #[test]
    fn test_get_when_team_member_not_found() -> Result<()> {
        let key = "lm";
        let args = &["config", "--global", &format!("coauthors.{key}")];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.get(key)?;

        assert_eq!(result, None);

        Ok(())
    }

    #[test]
    fn test_get_when_unexpected_error() -> Result<()> {
        let key = "lm";
        let args = &["config", "--global", &format!("coauthors.{key}")];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.get(key);

        assert!(
            result
                .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129")
        );

        Ok(())
    }

    #[test]
    fn test_get_when_terminated_by_signal() -> Result<()> {
        let key = "lm";
        let args = &["config", "--global", &format!("coauthors.{key}")];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.get(key);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_remove() -> Result<()> {
        let key = "lm";
        let args = &[
            "config",
            "--global",
            "--unset-all",
            &format!("coauthors.{key}"),
        ];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        team_member_repo.remove(key)?;

        Ok(())
    }

    #[test]
    fn test_remove_when_team_member_not_found() -> Result<()> {
        let key = "lm";
        let args = &[
            "config",
            "--global",
            "--unset-all",
            &format!("coauthors.{key}"),
        ];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.remove(key);

        assert!(
            result.is_err_and(|x| x.to_string() == "Git config command exited with status code: 1")
        );

        Ok(())
    }

    #[test]
    fn test_remove_when_terminated_by_signal() -> Result<()> {
        let key = "lm";
        let args = &[
            "config",
            "--global",
            "--unset-all",
            &format!("coauthors.{key}"),
        ];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.remove(key);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_add() -> Result<()> {
        let key = "lm";
        let team_member = "Leo Messi <leo.messi@example.com>";
        let args = &[
            "config",
            "--global",
            &format!("coauthors.{key}"),
            team_member,
        ];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        team_member_repo.add(key, team_member)?;

        Ok(())
    }

    #[test]
    fn test_add_when_invalid_key() -> Result<()> {
        let key = "l_m";
        let team_member = "Leo Messi <leo.messi@example.com>";
        let args = &[
            "config",
            "--global",
            &format!("coauthors.{key}"),
            team_member,
        ];
        let stdout = vec![];
        let stderr = format!("error: invalid key: coauthors.{key}").into_bytes();
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.add(key, team_member);

        assert!(result.is_err_and(|x| x.to_string() == format!("Invalid key: {key}")));

        Ok(())
    }

    #[test]
    fn test_add_when_unexpected_error() -> Result<()> {
        let key = "lm";
        let team_member = "Leo Messi <leo.messi@example.com>";
        let args = &[
            "config",
            "--global",
            &format!("coauthors.{key}"),
            team_member,
        ];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };

        let result = team_member_repo.add(key, team_member);

        assert!(
            result
                .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129")
        );

        Ok(())
    }

    #[test]
    fn test_add_when_terminated_by_signal() -> Result<()> {
        let key = "lm";
        let team_member = "Leo Messi <leo.messi@example.com>";
        let args = &[
            "config",
            "--global",
            &format!("coauthors.{key}"),
            team_member,
        ];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let team_member_repo = GitConfigTeamMemberRepo { command_runner };
        let result = team_member_repo.add(key, team_member);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }
}

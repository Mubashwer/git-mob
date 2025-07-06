use crate::Result;
use crate::helpers::{CmdOutput, CommandRunner};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait MobSessionRepo {
    fn list_coauthors(&self) -> Result<Vec<String>>;
    fn add_coauthor(&self, coauthor: &str) -> Result<()>;
    fn clear(&self) -> Result<()>;
}

pub struct GitConfigMobRepo<Cmd> {
    pub command_runner: Cmd,
}

impl<Cmd: CommandRunner> GitConfigMobRepo<Cmd> {
    const COAUTHORS_MOB_SECTION: &'static str = "coauthors-mob";
    const COAUTHOR_MOB_KEY: &'static str = "entry";

    const EXIT_CODE_SUCCESS: i32 = 0;
    const EXIT_CODE_CONFIG_INVALID_KEY: i32 = 1;

    fn git_config_error<T>(output: &CmdOutput) -> Result<T> {
        match output.status_code {
            Some(code) => Err(format!("Git config command exited with status code: {code}").into()),
            None => Err("Git config command terminated by signal".into()),
        }
    }
}

impl<Cmd: CommandRunner> MobSessionRepo for GitConfigMobRepo<Cmd> {
    fn list_coauthors(&self) -> Result<Vec<String>> {
        let full_key = format!("{}.{}", Self::COAUTHORS_MOB_SECTION, Self::COAUTHOR_MOB_KEY);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", "--get-all", &full_key])?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(String::from_utf8(output.stdout)?
                .lines()
                .map(|x| x.into())
                .collect()),

            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Ok(vec![]),
            _ => Self::git_config_error(&output),
        }
    }
    fn add_coauthor(&self, coauthor: &str) -> Result<()> {
        let full_key = format!("{}.{}", Self::COAUTHORS_MOB_SECTION, Self::COAUTHOR_MOB_KEY);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", "--add", &full_key, coauthor])?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(()),
            _ => Self::git_config_error(&output),
        }
    }
    fn clear(&self) -> Result<()> {
        if self.list_coauthors()?.is_empty() {
            return Ok(());
        }

        let section = Self::COAUTHORS_MOB_SECTION.to_owned();

        let output = self
            .command_runner
            .execute("git", &["config", "--global", "--remove-section", &section])?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(()),
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
    fn test_list_coauthors() -> Result<()> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout =
            b"Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n".into();
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let mob_repo = GitConfigMobRepo { command_runner };

        let result = mob_repo.list_coauthors()?;

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
    fn test_list_coauthors_when_mob_session_empty() -> Result<()> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let mob_repo = GitConfigMobRepo { command_runner };

        let result = mob_repo.list_coauthors()?;

        assert_eq!(result, Vec::<String>::new());

        Ok(())
    }

    #[test]
    fn test_list_coauthors_when_unexpected_error() -> Result<()> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let mob_repo = GitConfigMobRepo { command_runner };

        let result = mob_repo.list_coauthors();

        assert!(
            result
                .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129")
        );

        Ok(())
    }

    #[test]
    fn test_list_coauthors_when_terminated_by_signal() -> Result<()> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let mob_repo = GitConfigMobRepo { command_runner };

        let result = mob_repo.list_coauthors();

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_add_coauthor() -> Result<()> {
        let coauthor = "Leo Messi <leo.messi@example.com>";
        let args = &[
            "config",
            "--global",
            "--add",
            "coauthors-mob.entry",
            coauthor,
        ];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let mob_repo = GitConfigMobRepo { command_runner };

        mob_repo.add_coauthor(coauthor)?;

        Ok(())
    }

    #[test]
    fn test_add_coauthor_when_unexpected_error() -> Result<()> {
        let coauthor = "Leo Messi <leo.messi@example.com>";
        let args = &[
            "config",
            "--global",
            "--add",
            "coauthors-mob.entry",
            coauthor,
        ];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let mob_repo = GitConfigMobRepo { command_runner };

        let result = mob_repo.add_coauthor(coauthor);

        assert!(
            result
                .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129")
        );

        Ok(())
    }

    #[test]
    fn test_add_coauthor_when_terminated_by_signal() -> Result<()> {
        let coauthor = "Leo Messi <leo.messi@example.com>";
        let args = &[
            "config",
            "--global",
            "--add",
            "coauthors-mob.entry",
            coauthor,
        ];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let mob_repo = GitConfigMobRepo { command_runner };

        let result = mob_repo.add_coauthor(coauthor);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_clear() -> Result<()> {
        let mut command_runner = MockCommandRunner::new();
        command_runner
            .expect_execute()
            .once()
            .withf(|program, args| program == "git" && args == ["config", "--global", "--get-all", "coauthors-mob.entry"])
            .returning(|_, _| {
                Ok(CmdOutput {
                    stdout: b"Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n".into(),
                    stderr: vec![],
                    status_code: Some(0),
                })
            });
        command_runner
            .expect_execute()
            .once()
            .withf(|program, args| {
                program == "git"
                    && args == ["config", "--global", "--remove-section", "coauthors-mob"]
            })
            .returning(|_, _| {
                Ok(CmdOutput {
                    stdout: vec![],
                    stderr: vec![],
                    status_code: Some(0),
                })
            });
        let mob_repo = GitConfigMobRepo { command_runner };

        mob_repo.clear()?;

        Ok(())
    }

    #[test]
    fn test_clear_when_mob_session_empty() -> Result<()> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let mob_repo = GitConfigMobRepo { command_runner };

        mob_repo.clear()?;

        Ok(())
    }

    #[test]
    fn test_clear_when_unexpected_error() -> Result<()> {
        let mut command_runner = MockCommandRunner::new();
        command_runner
            .expect_execute()
            .once()
            .withf( |program, args| program == "git" && args == ["config", "--global", "--get-all", "coauthors-mob.entry"])
            .returning( |_, _| {
                Ok(CmdOutput {
                    stdout: b"Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n".into(),
                    stderr: vec![],
                    status_code: Some(0),
                })
            });
        command_runner
            .expect_execute()
            .once()
            .withf(|program, args| {
                program == "git"
                    && args == ["config", "--global", "--remove-section", "coauthors-mob"]
            })
            .returning(|_, _| {
                Ok(CmdOutput {
                    stdout: vec![],
                    stderr: b"uh-oh!".into(),
                    status_code: Some(129),
                })
            });
        let mob_repo = GitConfigMobRepo { command_runner };

        let result = mob_repo.clear();

        assert!(
            result
                .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129")
        );

        Ok(())
    }

    #[test]
    fn test_clear_when_terminated_by_signal() -> Result<()> {
        let mut command_runner = MockCommandRunner::new();
        command_runner
            .expect_execute()
            .once()
            .withf( |program, args| program == "git" && args == ["config", "--global", "--get-all", "coauthors-mob.entry"])
            .returning( |_, _| {
                Ok(CmdOutput {
                    stdout: b"Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n".into(),
                    stderr: vec![],
                    status_code: Some(0),
                })
            });
        command_runner
            .expect_execute()
            .once()
            .withf(|program, args| {
                program == "git"
                    && args == ["config", "--global", "--remove-section", "coauthors-mob"]
            })
            .returning(|_, _| {
                Ok(CmdOutput {
                    stdout: vec![],
                    stderr: vec![],
                    status_code: None,
                })
            });
        let mob_repo = GitConfigMobRepo { command_runner };
        let result = mob_repo.clear();

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }
}

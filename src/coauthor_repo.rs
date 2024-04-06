use crate::helpers::{CmdOutput, CommandRunner};
use std::error::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CoauthorRepo {
    fn list(&self, show_keys: bool) -> Result<Vec<String>, Box<dyn Error>>;
    fn list_mob(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>>;
    fn remove(&self, key: &str) -> Result<(), Box<dyn Error>>;
    fn add(&self, key: &str, coauthor: &str) -> Result<(), Box<dyn Error>>;
    fn add_to_mob(&self, coauthor: &str) -> Result<(), Box<dyn Error>>;
    fn clear_mob(&self) -> Result<(), Box<dyn Error>>;
}

pub struct GitConfigCoauthorRepo<Cmd> {
    pub command_runner: Cmd,
}

impl<Cmd: CommandRunner> GitConfigCoauthorRepo<Cmd> {
    const COAUTHORS_SECTION: &'static str = "coauthors";
    const COAUTHORS_MOB_SECTION: &'static str = "coauthors-mob";
    const COAUTHOR_MOB_KEY: &'static str = "entry";

    const EXIT_CODE_SUCCESS: i32 = 0;
    const EXIT_CODE_CONFIG_INVALID_KEY: i32 = 1;

    fn git_config_error<T>(output: &CmdOutput) -> Result<T, Box<dyn Error>> {
        match output.status_code {
            Some(code) => Err(format!("Git config command exited with status code: {code}").into()),
            None => Err("Git config command terminated by signal".into()),
        }
    }
}

impl<Cmd: CommandRunner> CoauthorRepo for GitConfigCoauthorRepo<Cmd> {
    fn list(&self, show_keys: bool) -> Result<Vec<String>, Box<dyn Error>> {
        let section = Self::COAUTHORS_SECTION;
        let search_regex = format!("^{section}\\.");

        let output = self.command_runner.execute(
            "git",
            &["config", "--global", "--get-regexp", &search_regex],
        )?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => String::from_utf8(output.stdout)?
                .lines()
                .map(|x| {
                    let delimiter = match show_keys {
                        true => format!("{section}."),
                        false => " ".to_owned(),
                    };
                    x.split_once(&delimiter)
                        .ok_or(format!("Failed to split string: '{x}'").into())
                        .map(|(_, coauthor)| coauthor.to_owned())
                })
                .collect(),

            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Ok(vec![]),
            _ => Self::git_config_error(&output),
        }
    }

    fn list_mob(&self) -> Result<Vec<String>, Box<dyn Error>> {
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

    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", &full_key])?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => {
                Ok(Some(String::from_utf8(output.stdout)?.trim().into()))
            }
            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Ok(None),
            _ => Self::git_config_error(&output),
        }
    }

    fn remove(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", "--unset-all", &full_key])?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(()),
            _ => Self::git_config_error(&output),
        }
    }

    fn add(&self, key: &str, coauthor: &str) -> Result<(), Box<dyn Error>> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", &full_key, coauthor])?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(()),
            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Err(format!("Invalid key: {key}").into()),
            _ => Self::git_config_error(&output),
        }
    }

    fn add_to_mob(&self, coauthor: &str) -> Result<(), Box<dyn Error>> {
        let full_key = format!("{}.{}", Self::COAUTHORS_MOB_SECTION, Self::COAUTHOR_MOB_KEY);

        let output = self
            .command_runner
            .execute("git", &["config", "--global", "--add", &full_key, coauthor])?;

        match output.status_code {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(()),
            _ => Self::git_config_error(&output),
        }
    }

    fn clear_mob(&self) -> Result<(), Box<dyn Error>> {
        if self.list_mob()?.is_empty() {
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
    fn test_list_coauthor_without_keys() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = b"coauthors.lm Leo Messi <leo.messi@example.com>\ncoauthors.em Emi Martinez <emi.martinez@example.com>\n".into();
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list(false)?;

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
    fn test_list_coauthor_with_keys() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = b"coauthors.lm Leo Messi <leo.messi@example.com>\ncoauthors.em Emi Martinez <emi.martinez@example.com>\n".into();
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list(true)?;

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
    fn test_list_coauthor_when_no_coauthors_added() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list(true)?;

        assert_eq!(result, Vec::<String>::new());

        Ok(())
    }

    #[test]
    fn test_list_coauthor_when_unexpected_error() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list(true);

        assert!(result
            .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129"));

        Ok(())
    }

    #[test]
    fn test_list_coauthor_when_terminated_by_signal() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-regexp", "^coauthors\\."];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list(true);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_list_mob() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout =
            b"Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n".into();
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list_mob()?;

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
    fn test_list_mob_when_mob_session_empty() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list_mob()?;

        assert_eq!(result, Vec::<String>::new());

        Ok(())
    }

    #[test]
    fn test_list_mob_when_unexpected_error() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list_mob();

        assert!(result
            .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129"));

        Ok(())
    }

    #[test]
    fn test_list_mob_when_terminated_by_signal() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.list_mob();

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let args = &["config", "--global", &format!("coauthors.{key}")];
        let stdout = b"Leo Messi <leo.messi@example.com>\n".into();
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.get(key)?;

        assert_eq!(result, Some("Leo Messi <leo.messi@example.com>".into()));

        Ok(())
    }

    #[test]
    fn test_get_when_coauthor_not_found() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let args = &["config", "--global", &format!("coauthors.{key}")];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.get(key)?;

        assert_eq!(result, None);

        Ok(())
    }

    #[test]
    fn test_get_when_unexpected_error() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let args = &["config", "--global", &format!("coauthors.{key}")];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.get(key);

        assert!(result
            .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129"));

        Ok(())
    }

    #[test]
    fn test_get_when_terminated_by_signal() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let args = &["config", "--global", &format!("coauthors.{key}")];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.get(key);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_remove() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        coauthor_repo.remove(key)?;

        Ok(())
    }

    #[test]
    fn test_remove_when_coauthor_not_found() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.remove(key);

        assert!(
            result.is_err_and(|x| x.to_string() == "Git config command exited with status code: 1")
        );

        Ok(())
    }

    #[test]
    fn test_remove_when_terminated_by_signal() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.remove(key);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let coauthor = "Leo Messi <leo.messi@example.com>";
        let args = &["config", "--global", &format!("coauthors.{key}"), coauthor];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(0);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        coauthor_repo.add(key, coauthor)?;

        Ok(())
    }

    #[test]
    fn test_add_when_invalid_key() -> Result<(), Box<dyn std::error::Error>> {
        let key = "l_m";
        let coauthor = "Leo Messi <leo.messi@example.com>";
        let args = &["config", "--global", &format!("coauthors.{key}"), coauthor];
        let stdout = vec![];
        let stderr = format!("error: invalid key: coauthors.{key}").into_bytes();
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.add(key, coauthor);

        assert!(result.is_err_and(|x| x.to_string() == format!("Invalid key: {key}")));

        Ok(())
    }

    #[test]
    fn test_add_when_unexpected_error() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let coauthor = "Leo Messi <leo.messi@example.com>";
        let args = &["config", "--global", &format!("coauthors.{key}"), coauthor];
        let stdout = vec![];
        let stderr = b"uh-oh!".into();
        let status_code = Some(129);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.add(key, coauthor);

        assert!(result
            .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129"));

        Ok(())
    }

    #[test]
    fn test_add_when_terminated_by_signal() -> Result<(), Box<dyn std::error::Error>> {
        let key = "lm";
        let coauthor = "Leo Messi <leo.messi@example.com>";
        let args = &["config", "--global", &format!("coauthors.{key}"), coauthor];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = None;
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.add(key, coauthor);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_add_to_mob() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        coauthor_repo.add_to_mob(coauthor)?;

        Ok(())
    }

    #[test]
    fn test_add_to_mob_when_unexpected_error() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.add_to_mob(coauthor);

        assert!(result
            .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129"));

        Ok(())
    }

    #[test]
    fn test_add_to_mob_when_terminated_by_signal() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.add_to_mob(coauthor);

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }

    #[test]
    fn test_clear_mob() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        coauthor_repo.clear_mob()?;

        Ok(())
    }

    #[test]
    fn test_clear_mob_when_mob_session_empty() -> Result<(), Box<dyn std::error::Error>> {
        let args = &["config", "--global", "--get-all", "coauthors-mob.entry"];
        let stdout = vec![];
        let stderr = vec![];
        let status_code = Some(1);
        let command_runner = create_mock_command_runner("git", args, stdout, stderr, status_code);
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        coauthor_repo.clear_mob()?;

        Ok(())
    }

    #[test]
    fn test_clear_mob_when_unexpected_error() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };

        let result = coauthor_repo.clear_mob();

        assert!(result
            .is_err_and(|x| x.to_string() == "Git config command exited with status code: 129"));

        Ok(())
    }

    #[test]
    fn test_clear_mob_when_terminated_by_signal() -> Result<(), Box<dyn std::error::Error>> {
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
        let coauthor_repo = GitConfigCoauthorRepo { command_runner };
        let result = coauthor_repo.clear_mob();

        assert!(result.is_err_and(|x| x.to_string() == "Git config command terminated by signal"));

        Ok(())
    }
}

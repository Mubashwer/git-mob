use crate::Result;
use std::process::Command;

pub struct CmdOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status_code: Option<i32>,
}

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CommandRunner {
    #[allow(clippy::needless_lifetimes)] // explicit lifetime is needed for automock
    fn execute<'a>(&self, program: &str, args: &[&'a str]) -> Result<CmdOutput>;
}

pub struct StdCommandRunner;

impl CommandRunner for StdCommandRunner {
    fn execute(&self, program: &str, args: &[&str]) -> Result<CmdOutput> {
        let output = Command::new(program).args(args).output()?;

        Ok(CmdOutput {
            stdout: output.stdout,
            stderr: output.stderr,
            status_code: output.status.code(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_success() -> Result<()> {
        let result = StdCommandRunner.execute("git", &["--version"])?;

        assert_eq!(result.status_code, Some(0));
        assert!(result.stdout.starts_with(b"git version "));
        assert!(result.stderr.is_empty());

        Ok(())
    }

    #[test]
    fn test_execute_failure() -> Result<()> {
        let result = StdCommandRunner.execute("git", &["--invalid_option"])?;

        assert!(result.status_code.is_some_and(|x| x != 0));
        assert!(result.stdout.is_empty());
        assert!(
            result
                .stderr
                .starts_with(b"unknown option: --invalid_option")
        );

        Ok(())
    }

    #[test]
    fn test_execute_error() -> Result<()> {
        let result = StdCommandRunner.execute("non_existent_program", &[]);

        assert!(result.is_err());

        Ok(())
    }
}

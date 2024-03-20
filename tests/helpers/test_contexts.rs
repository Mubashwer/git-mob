use std::{env, ffi::OsString, path::PathBuf, process::Command};

use assert_cmd::cargo::cargo_bin;
use once_cell::sync::Lazy;
use tempfile::NamedTempFile;
use test_context::TestContext;

static PATH_ENV_VAR: Lazy<OsString> = Lazy::new(|| {
    let path = &env::var_os("PATH").unwrap_or_default();

    // adding git-mob executable to the PATH so that it can be executed as "git mob"
    let exe_path = cargo_bin("git-mob");
    let exe_dir = exe_path.parent().unwrap();

    let mut split_paths: Vec<PathBuf> = env::split_paths(path).collect();
    split_paths.push(PathBuf::from(exe_dir));

    let new_path = env::join_paths(split_paths).unwrap();
    new_path
});

pub(crate) struct TestContextCli {
    git_config_global: NamedTempFile,
}

impl TestContextCli {
    pub fn git(&self) -> Command {
        let mut command = Command::new("git");
        command.env("GIT_CONFIG_GLOBAL", self.git_config_global.path());
        command
    }
}

impl TestContext for TestContextCli {
    fn setup() -> TestContextCli {
        env::set_var("PATH", &*PATH_ENV_VAR);

        TestContextCli {
            git_config_global: NamedTempFile::new().unwrap(),
        }
    }
}

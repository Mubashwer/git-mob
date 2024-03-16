use once_cell::sync::Lazy;
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::Command;
use tempfile::{NamedTempFile};

static PATH_ENV_VAR: Lazy<OsString> = Lazy::new(|| {
    // add target directory to PATH so that git-mob is available as "git mob"
    let target_dir = Path::new(env!("CARGO_BIN_EXE_git-mob")).parent().unwrap();
    let mut path = OsString::from(target_dir);
    path.push(";");
    path.push(env::var_os("PATH").expect("PATH env var should be set"));
    path
});

pub struct TestContext {
    git_config_global: NamedTempFile,
}

impl TestContext {
    pub fn new() -> TestContext {
        TestContext {
            git_config_global: NamedTempFile::new().unwrap(),
        }
    }

    pub fn git(&self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Command {
        let mut command = Command::new("git");
        command
            .env("GIT_CONFIG_GLOBAL", self.git_config_global.path())
            .env("PATH", &*PATH_ENV_VAR)
            .args(args);
        command
    }
}

use std::{env, ffi::OsString, path::PathBuf, process::Command};

use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin};
use once_cell::sync::Lazy;
use tempfile::{NamedTempFile, TempDir, TempPath, tempdir};
use test_context::TestContext;

static PATH_WITH_GIT_MOB: Lazy<OsString> = Lazy::new(|| {
    let path = &env::var_os("PATH").unwrap_or_default();

    // adding git-mob executable to the PATH so that it can be executed as "git mob"
    let exe_path = cargo_bin("git-mob");
    let exe_dir = exe_path.parent().unwrap();

    let mut split_paths: Vec<PathBuf> = env::split_paths(path).collect();
    split_paths.push(PathBuf::from(exe_dir));

    env::join_paths(split_paths).unwrap()
});

pub(crate) struct TestContextCli {
    git_config_global: TempPath,
}

impl TestContextCli {
    #[allow(dead_code)] // incorrectly detected as unused by rustc; used in tests
    pub fn git(&self) -> Command {
        let mut command = Command::new("git");
        command.env("PATH", &*PATH_WITH_GIT_MOB);
        command.env("GIT_CONFIG_GLOBAL", &self.git_config_global);
        command
    }
}

impl TestContext for TestContextCli {
    fn setup() -> TestContextCli {
        TestContextCli {
            git_config_global: NamedTempFile::new().unwrap().into_temp_path(),
        }
    }
}

pub(crate) struct TestContextRepo {
    git_config_global: TempPath,
    dir: TempDir,
    pub home_dir: TempDir,
}

impl TestContextRepo {
    pub fn git(&self) -> Command {
        let mut command = Command::new("git");
        command
            .current_dir(self.dir.path())
            .env("PATH", &*PATH_WITH_GIT_MOB)
            .env("GIT_CONFIG_GLOBAL", &self.git_config_global);

        #[cfg(unix)]
        {
            command.env("HOME", self.home_dir.path());
        }
        #[cfg(windows)]
        {
            command.env("USERPROFILE", self.home_dir.path());
        }

        command
    }
}

impl TestContext for TestContextRepo {
    fn setup() -> TestContextRepo {
        let ctx = TestContextRepo {
            git_config_global: NamedTempFile::new().unwrap().into_temp_path(),
            dir: tempdir().unwrap(),
            home_dir: tempdir().unwrap(),
        };

        ctx.git().arg("init").assert().success();
        ctx.git()
            .args(["config", "--global", "user.name", "Cata Diaz"])
            .assert()
            .success();
        ctx.git()
            .args(["config", "--global", "user.email", "cata.diaz@example.com"])
            .assert()
            .success();

        ctx
    }
}

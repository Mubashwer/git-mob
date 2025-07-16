mod helpers;

use assert_cmd::prelude::*;
use helpers::test_contexts::TestContextRepo;
use predicates::prelude::*;
use std::{error::Error, fs, path::Path};
use tempfile::TempDir;
use test_context::test_context;

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_given_hooks_dir_not_set(ctx: TestContextRepo) -> Result<(), Box<dyn Error>> {
    let hooks_dir = ctx.home_dir.path().join(".git").join("hooks");

    ctx.git()
        .args(["mob", "setup"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Set global githooks directory: {}
Created new prepare-commit-msg githook: {}
Setup complete
"#,
            hooks_dir.to_string_lossy(),
            hooks_dir.join("prepare-commit-msg").to_string_lossy()
        )));

    verify_prepare_commit_msg_global_hook(&ctx, &hooks_dir)
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_given_hooks_dir_set_and_exists(ctx: TestContextRepo) -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let hooks_dir = temp_dir.path().to_path_buf();

    // setting global hooks directory
    ctx.git()
        .args([
            "config",
            "--global",
            "core.hooksPath",
            &hooks_dir.to_string_lossy(),
        ])
        .assert()
        .success();

    ctx.git()
        .args(["mob", "setup"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Created new prepare-commit-msg githook: {}
Setup complete
"#,
            hooks_dir.join("prepare-commit-msg").to_string_lossy()
        )));

    verify_prepare_commit_msg_global_hook(&ctx, &hooks_dir)
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_given_hooks_dir_set_but_does_not_exist(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let hooks_dir = temp_dir.path().to_path_buf();

    // setting global hooks directory
    ctx.git()
        .args([
            "config",
            "--global",
            "core.hooksPath",
            &hooks_dir.to_string_lossy(),
        ])
        .assert()
        .success();

    // removing the global hooks directory
    temp_dir.close()?;
    assert!(!hooks_dir.exists());

    ctx.git()
        .args(["mob", "setup"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Created new prepare-commit-msg githook: {}
Setup complete
"#,
            hooks_dir.join("prepare-commit-msg").to_string_lossy()
        )));

    verify_prepare_commit_msg_global_hook(&ctx, &hooks_dir)
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_given_hooks_dir_set_and_starts_with_tilde(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    let hooks_dir = Path::new("~/my/githooks");
    let expanded_hooks_dir = ctx.home_dir.path().join("my").join("githooks");

    // setting global hooks directory
    ctx.git()
        .args([
            "config",
            "--global",
            "core.hooksPath",
            &hooks_dir.to_string_lossy(),
        ])
        .assert()
        .success();

    ctx.git()
        .args(["mob", "setup"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Created new prepare-commit-msg githook: {}
Setup complete
"#,
            expanded_hooks_dir
                .join("prepare-commit-msg")
                .to_string_lossy()
        )));

    verify_prepare_commit_msg_global_hook(&ctx, hooks_dir)
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_given_prepare_commit_msg_hook_already_exists(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let hooks_dir = temp_dir.path().to_path_buf();

    let hook_path = hooks_dir.join("prepare-commit-msg");
    let backup_path = hooks_dir.join("prepare-commit-msg.bak");

    let existing_hook_contents = "#Lorem ipsum";
    fs::write(&hook_path, existing_hook_contents.as_bytes())?;

    // setting global hooks directory
    ctx.git()
        .args([
            "config",
            "--global",
            "core.hooksPath",
            &hooks_dir.to_string_lossy(),
        ])
        .assert()
        .success();

    ctx.git()
        .args(["mob", "setup"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Backed up existing prepare-commit-msg githook: {}
Created new prepare-commit-msg githook: {}
Setup complete
"#,
            backup_path.to_string_lossy(),
            hook_path.to_string_lossy()
        )));

    // verifying existing prepare-commit-msg is backed up as prepare-commit-msg.bak
    assert!(backup_path.exists());
    assert!(fs::metadata(&backup_path)?.is_file());

    let backup_contents = fs::read_to_string(&backup_path)?;
    assert_eq!(backup_contents, existing_hook_contents);

    verify_prepare_commit_msg_global_hook(&ctx, &hooks_dir)
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_given_invalid_git_config_global_path(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .env("GIT_CONFIG_GLOBAL", "")
        .args(["mob", "setup"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: Failed to set global githooks directory to",
        ));

    Ok(())
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_local_given_hooks_dir_not_set(ctx: TestContextRepo) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "setup", "--local"])
        .assert()
        .failure()
        .stderr("Error: Local githooks directory is not set\n");

    Ok(())
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_local_given_hooks_dir_set_and_exists(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let hooks_dir = temp_dir.path().to_path_buf();

    // setting local hooks directory
    ctx.git()
        .args([
            "config",
            "--local",
            "core.hooksPath",
            &hooks_dir.to_string_lossy(),
        ])
        .assert()
        .success();

    ctx.git()
        .args(["mob", "setup", "--local"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Created new prepare-commit-msg githook: {}
Setup complete
"#,
            hooks_dir.join("prepare-commit-msg").to_string_lossy()
        )));

    verify_prepare_commit_msg_local_hook(&ctx, &hooks_dir)
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_local_given_hooks_dir_set_but_does_not_exist(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let hooks_dir = temp_dir.path().to_path_buf();

    // setting global hooks directory
    ctx.git()
        .args([
            "config",
            "--local",
            "core.hooksPath",
            &hooks_dir.to_string_lossy(),
        ])
        .assert()
        .success();

    // removing the local hooks directory
    temp_dir.close()?;
    assert!(!hooks_dir.exists());

    ctx.git()
        .args(["mob", "setup", "--local"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Created new prepare-commit-msg githook: {}
Setup complete
"#,
            hooks_dir.join("prepare-commit-msg").to_string_lossy()
        )));

    verify_prepare_commit_msg_local_hook(&ctx, &hooks_dir)
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_local_given_hooks_dir_set_and_starts_with_tilde(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    let hooks_dir = Path::new("~/my/githooks");
    let expanded_hooks_dir = ctx.home_dir.path().join("my").join("githooks");

    // setting local hooks directory
    ctx.git()
        .args([
            "config",
            "--local",
            "core.hooksPath",
            &hooks_dir.to_string_lossy(),
        ])
        .assert()
        .success();

    ctx.git()
        .args(["mob", "setup", "--local"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Created new prepare-commit-msg githook: {}
Setup complete
"#,
            expanded_hooks_dir
                .join("prepare-commit-msg")
                .to_string_lossy()
        )));

    verify_prepare_commit_msg_local_hook(&ctx, hooks_dir)
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_setup_local_given_prepare_commit_msg_hook_already_exists(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let hooks_dir = temp_dir.path().to_path_buf();

    let hook_path = hooks_dir.join("prepare-commit-msg");
    let backup_path = hooks_dir.join("prepare-commit-msg.bak");

    let existing_hook_contents = "#Lorem ipsum";
    fs::write(&hook_path, existing_hook_contents.as_bytes())?;

    // setting local hooks directory
    ctx.git()
        .args([
            "config",
            "--local",
            "core.hooksPath",
            &hooks_dir.to_string_lossy(),
        ])
        .assert()
        .success();

    ctx.git()
        .args(["mob", "setup", "--local"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            r#"Backed up existing prepare-commit-msg githook: {}
Created new prepare-commit-msg githook: {}
Setup complete
"#,
            backup_path.to_string_lossy(),
            hook_path.to_string_lossy()
        )));

    // verifying existing prepare-commit-msg is backed up as prepare-commit-msg.bak
    assert!(backup_path.exists());
    assert!(fs::metadata(&backup_path)?.is_file());

    let backup_contents = fs::read_to_string(&backup_path)?;
    assert_eq!(backup_contents, existing_hook_contents);

    verify_prepare_commit_msg_local_hook(&ctx, &hooks_dir)
}

fn verify_prepare_commit_msg_global_hook(
    ctx: &TestContextRepo,
    hooks_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    // verifying global hooks directory
    ctx.git()
        .args(["config", "--global", "core.hooksPath"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            "{}\n",
            hooks_dir.to_string_lossy()
        )));

    let hook_path = if hooks_dir.starts_with("~") {
        let mut expanded_hooks_dir = ctx.home_dir.path().to_path_buf();
        expanded_hooks_dir.extend(hooks_dir.components().skip(1));
        expanded_hooks_dir.join("prepare-commit-msg")
    } else {
        hooks_dir.join("prepare-commit-msg")
    };

    // verifying prepare-commit-msg githook exists
    assert!(hook_path.exists());

    let metadata = fs::metadata(&hook_path)?;
    assert!(metadata.is_file());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(metadata.permissions().mode() & 0o777, 0o755);
    }

    // verifying prepare-commit-msg githook contents
    assert_eq!(
        fs::read_to_string(&hook_path)?,
        include_str!("../src/commands/prepare-commit-msg")
    );

    Ok(())
}

fn verify_prepare_commit_msg_local_hook(
    ctx: &TestContextRepo,
    hooks_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    // verifying local hooks directory
    ctx.git()
        .args(["config", "--local", "core.hooksPath"])
        .assert()
        .success()
        .stdout(predicate::str::diff(format!(
            "{}\n",
            hooks_dir.to_string_lossy()
        )));

    let hook_path = if hooks_dir.starts_with("~") {
        let mut expanded_hooks_dir = ctx.home_dir.path().to_path_buf();
        expanded_hooks_dir.extend(hooks_dir.components().skip(1));
        expanded_hooks_dir.join("prepare-commit-msg")
    } else {
        hooks_dir.join("prepare-commit-msg")
    };

    // verifying prepare-commit-msg githook exists
    assert!(hook_path.exists());

    let metadata = fs::metadata(&hook_path)?;
    assert!(metadata.is_file());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(metadata.permissions().mode() & 0o777, 0o755);
    }

    // verifying prepare-commit-msg githook contents
    assert_eq!(
        fs::read_to_string(&hook_path)?,
        include_str!("../src/commands/prepare-commit-msg.local")
    );

    Ok(())
}

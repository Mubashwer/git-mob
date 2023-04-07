use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use std::{env, process::Command};

fn before_each() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("git")
        .args(["config", "--global", "--remove-section", "coauthors"])
        .output()
        .expect("failed to execute process");

    Command::new("git")
        .args(["config", "--global", "--remove-section", "coauthors-mob"])
        .output()
        .expect("failed to execute process");

    // adding 2 co-authors
    let mut cmd = Command::cargo_bin("git-mob")?;
    cmd.args([
        "coauthor",
        "--add",
        "lm",
        "Leo Messi",
        "leo.messi@example.com",
    ]);
    cmd.assert().success();

    cmd = Command::cargo_bin("git-mob")?;
    cmd.args([
        "coauthor",
        "--add",
        "em",
        "Emi Martinez",
        "emi.martinez@example.com",
    ]);
    cmd.assert().success();

    // set global githooks directory to current directory
    // so it can find prepare-commit-msg
    let current_dir = env::current_dir()?.display().to_string();
    Command::new("git")
        .args(["config", "--global", "core.hooksPath", &current_dir])
        .output()
        .expect("failed to execute process");

    Ok(())
}

fn assert_commit_message(input_msg: &str, expected_msg: &str) {
    let mut cmd = Command::new("git");
    cmd.args(["config", "user.email", "test@email.com"]);
    cmd.assert().success();

    let mut cmd = Command::new("git");
    cmd.args(["config", "user.name", "Test User"]);
    cmd.assert().success();
    
    
    // making a test commit
    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", input_msg, "--allow-empty"]);
    cmd.assert().success();

    // asserting test commit has the expected message
    // note: git show displays the commit message with an indent of 4 spaces
    cmd = Command::new("git");
    cmd.args(["show", "--name-only"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::ends_with(expected_msg));

    cmd = Command::new("git");
    cmd.args(["reset", "--soft", "HEAD~1"]);
    cmd.assert().success();
}

#[test]
#[serial]
fn test_mob_with_by_keys() -> Result<(), Box<dyn std::error::Error>> {
    before_each()?;

    // mobbing with both of the co-authors
    let mut cmd = Command::cargo_bin("git-mob")?;
    cmd.args(["--with", "lm", "em"]);
    cmd.assert().success().stdout(predicate::str::diff(
        "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
    ));

    // verifying mob list shows both of the co-authors
    cmd = Command::cargo_bin("git-mob")?;
    cmd.args(["--list"]);
    cmd.assert().success().stdout(predicate::str::diff(
        "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
    ));

    let input_commit_msg = "making a test commit";
    let expected_commit_msg = r#"
    making a test commit
    
    Co-authored-by: Leo Messi <leo.messi@example.com>
    Co-authored-by: Emi Martinez <emi.martinez@example.com>
"#;

    assert_commit_message(input_commit_msg, expected_commit_msg);

    Ok(())
}

#[test]
#[serial]
fn test_mob_clear() -> Result<(), Box<dyn std::error::Error>> {
    before_each()?;

    // mobbing with both of the co-authors
    let mut cmd = Command::cargo_bin("git-mob")?;
    cmd.args(["--with", "lm", "em"]);
    cmd.assert().success().stdout(predicate::str::diff(
        "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
    ));

    // clearing mob session
    cmd = Command::cargo_bin("git-mob")?;
    cmd.args(["--clear"]);
    cmd.assert().success();

    // verifying mob list is empty
    cmd = Command::cargo_bin("git-mob")?;
    cmd.args(["--list"]);
    cmd.assert().success().stdout(predicate::str::diff(""));

    let input_commit_msg = "making a test commit";
    let expected_commit_msg = r#"
    making a test commit
"#;

    assert_commit_message(input_commit_msg, expected_commit_msg);

    Ok(())
}

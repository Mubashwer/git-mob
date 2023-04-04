use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use std::process::Command;

fn before_each() {
    Command::new("git")
        .args(["config", "--global", "--remove-section", "coauthors"])
        .output()
        .expect("failed to execute process");
}

#[test]
#[serial]
fn test_add_coauthors() -> Result<(), Box<dyn std::error::Error>> {
    before_each();

    // adding 2 co-authors
    let mut cmd = Command::cargo_bin("git-mob")?;
    cmd.args([
        "coauthor",
        "--add",
        "lm",
        "Leo Messi",
        "leo.messi@example.com",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::diff("Leo Messi <leo.messi@example.com>\n"));

    cmd = Command::cargo_bin("git-mob")?;
    cmd.args([
        "coauthor",
        "--add",
        "em",
        "Emi Martinez",
        "emi.martinez@example.com",
    ]);
    cmd.assert().success().stdout(predicate::str::diff(
        "Emi Martinez <emi.martinez@example.com>\n",
    ));

    // co-authors list shows the 2 co-authors that were added
    let mut cmd = Command::cargo_bin("git-mob")?;
    cmd.args(["coauthor", "--list"]);
    cmd.assert().success().stdout(predicate::str::diff(
        "lm Leo Messi <leo.messi@example.com>\nem Emi Martinez <emi.martinez@example.com>\n",
    ));

    Ok(())
}

#[test]
#[serial]
fn test_delete_coauthor() -> Result<(), Box<dyn std::error::Error>> {
    before_each();

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

    // deleting one co-author
    cmd = Command::cargo_bin("git-mob")?;
    cmd.args(["coauthor", "--delete", "lm"]);
    cmd.assert().success();

    // co-authors list excludes the deleted co-author
    cmd = Command::cargo_bin("git-mob")?;
    cmd.args(["coauthor", "--list"]);
    cmd.assert().success().stdout(predicate::str::diff(
        "em Emi Martinez <emi.martinez@example.com>\n",
    ));

    Ok(())
}

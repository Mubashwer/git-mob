use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use std::process::Command;

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

    Ok(())
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

    Ok(())
}

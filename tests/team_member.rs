mod helpers;

use assert_cmd::prelude::*;
use helpers::test_contexts::TestContextCli;
use predicates::prelude::*;
use std::error::Error;
use test_context::test_context;

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_add_and_list_team_members(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // adding 2 team members
    ctx.git()
        .args([
            "mob",
            "team-member",
            "--add",
            "lm",
            "Leo Messi",
            "leo.messi@example.com",
        ])
        .assert()
        .success()
        .stdout(predicate::str::diff("Leo Messi <leo.messi@example.com>\n"));

    ctx.git()
        .args([
            "mob",
            "team-member",
            "--add",
            "em",
            "Emi Martinez",
            "emi.martinez@example.com",
        ])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Emi Martinez <emi.martinez@example.com>\n",
        ));

    // team members list shows the 2 team members that were added
    ctx.git()
        .args(["mob", "team-member", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "lm Leo Messi <leo.messi@example.com>\nem Emi Martinez <emi.martinez@example.com>\n",
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_add_member_with_invalid_key(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args([
            "mob",
            "team-member",
            "--add",
            "invalid_key_with_underscore",
            "Leo Messi",
            "leo.messi@example.com",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::diff(
            "Error: \"Invalid key: invalid_key_with_underscore\"\n",
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_list_team_members_given_no_team_members_added(
    ctx: TestContextCli,
) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "team-member", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(""));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_delete_team_members(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // adding 2 team members
    ctx.git()
        .args([
            "mob",
            "team-member",
            "--add",
            "lm",
            "Leo Messi",
            "leo.messi@example.com",
        ])
        .assert()
        .success();

    ctx.git()
        .args([
            "mob",
            "team-member",
            "--add",
            "em",
            "Emi Martinez",
            "emi.martinez@example.com",
        ])
        .assert()
        .success();

    // deleting one team member
    ctx.git()
        .args(["mob", "team-member", "--delete", "lm"])
        .assert()
        .success();

    // team members list excludes the deleted team member
    ctx.git()
        .args(["mob", "team-member", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "em Emi Martinez <emi.martinez@example.com>\n",
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_delete_member_when_member_not_found(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "team-member", "--delete", "lm"])
        .assert()
        .failure()
        .stderr(predicate::str::diff(
            "Error: \"No team member found with key: lm\"\n",
        ));

    Ok(())
}

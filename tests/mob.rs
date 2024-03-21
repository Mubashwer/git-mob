mod helpers;

use assert_cmd::prelude::*;
use helpers::test_contexts::TestContextCli;
use predicates::prelude::*;
use std::error::Error;
use test_context::test_context;

fn before_each(ctx: &TestContextCli) -> Result<(), Box<dyn Error>> {
    // adding 2 co-authors
    ctx.git()
        .args([
            "mob",
            "coauthor",
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
            "coauthor",
            "--add",
            "em",
            "Emi Martinez",
            "emi.martinez@example.com",
        ])
        .assert()
        .success();

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_with_by_keys(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    before_each(&ctx)?;

    // mobbing with both of the co-authors
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // verifying mob list shows both of the co-authors
    ctx.git()
        .args(["mob", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_clear(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    before_each(&ctx)?;

    // mobbing with both of the co-authors
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // clearing mob session
    ctx.git().args(["mob", "--clear"]).assert().success();

    // verifying mob list is empty
    ctx.git()
        .args(["mob", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(""));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_coauthor_trailers(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    before_each(&ctx)?;

    // mobbing with both of the co-authors
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // verifying mob trailers show Co-authored-by trailers for both co-authors
    ctx.git()
        .args(["mob", "--trailers"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Co-authored-by: Leo Messi <leo.messi@example.com>\nCo-authored-by: Emi Martinez <emi.martinez@example.com>\n",
        ));

    Ok(())
}

mod helpers;

use assert_cmd::prelude::*;
use helpers::test_contexts::TestContextCli;
use predicates::prelude::*;
use std::error::Error;
use test_context::test_context;

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_add_and_list_coauthors(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
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
        .success()
        .stdout(predicate::str::diff("Leo Messi <leo.messi@example.com>\n"));

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
        .success()
        .stdout(predicate::str::diff(
            "Emi Martinez <emi.martinez@example.com>\n",
        ));

    // co-authors list shows the 2 co-authors that were added
    ctx.git()
        .args(["mob", "coauthor", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "lm Leo Messi <leo.messi@example.com>\nem Emi Martinez <emi.martinez@example.com>\n",
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_add_coauthor_with_invalid_key(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args([
            "mob",
            "coauthor",
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
fn test_list_coauthors_given_no_coauthors_added(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "coauthor", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(""));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_delete_coauthor(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
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

    // deleting one co-author
    ctx.git()
        .args(["mob", "coauthor", "--delete", "lm"])
        .assert()
        .success();

    // co-authors list excludes the deleted co-author
    ctx.git()
        .args(["mob", "coauthor", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "em Emi Martinez <emi.martinez@example.com>\n",
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_delete_coauthor_when_coauthor_not_found(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "coauthor", "--delete", "lm"])
        .assert()
        .failure()
        .stderr(predicate::str::diff(
            "Error: \"No co-author found with key: lm\"\n",
        ));

    Ok(())
}

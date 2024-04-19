mod helpers;

use assert_cmd::prelude::*;
use helpers::test_contexts::TestContextRepo;
use predicates::prelude::*;
use std::error::Error;
use tempfile::TempDir;
use test_context::test_context;

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_prepare_commit_msg(ctx: TestContextRepo) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "setup", "--global"])
        .assert()
        .success();

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

    // mobbing with both of the co-authors
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\n\
             Emi Martinez <emi.martinez@example.com>\n",
        ));

    // git commit prints Co-authored-by trailers and an empty line to stderr
    ctx.git()
        .args(["commit", "--allow-empty", "--message", "test: hello world!"])
        .assert()
        .success()
        .stderr(predicate::str::diff(
            "Co-authored-by: Leo Messi <leo.messi@example.com>\n\
             Co-authored-by: Emi Martinez <emi.martinez@example.com>\n\n",
        ));

    // the commit body has message with Co-authored-by trailers
    ctx.git()
        .args(["show", "--no-patch", "--format=%B"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "test: hello world!\n\n\
            Co-authored-by: Leo Messi <leo.messi@example.com>\n\
            Co-authored-by: Emi Martinez <emi.martinez@example.com>\n\n",
        ));

    Ok(())
}

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_prepare_commit_msg_given_local_hooks_directory(
    ctx: TestContextRepo,
) -> Result<(), Box<dyn Error>> {
    let hooks_dir = TempDir::new()?;

    // setting local hooks directory
    ctx.git()
        .args([
            "config",
            "--local",
            "core.hooksPath",
            &hooks_dir.path().to_string_lossy(),
        ])
        .assert()
        .success();

    ctx.git()
        .args(["mob", "setup", "--global"])
        .assert()
        .success();

    // setup git mob locally - adds a local prepare-commit-msg hook which invokes the global one
    ctx.git()
        .args(["mob", "setup", "--local"])
        .assert()
        .success();

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

    // mobbing with both of the co-authors
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\n\
             Emi Martinez <emi.martinez@example.com>\n",
        ));

    // git commit prints Co-authored-by trailers and an empty line to stderr
    ctx.git()
        .args(["commit", "--allow-empty", "--message", "test: hello world!"])
        .assert()
        .success()
        .stderr(predicate::str::diff(
            "Co-authored-by: Leo Messi <leo.messi@example.com>\n\
             Co-authored-by: Emi Martinez <emi.martinez@example.com>\n\n",
        ));

    // the commit body has message with Co-authored-by trailers
    ctx.git()
        .args(["show", "--no-patch", "--format=%B"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "test: hello world!\n\n\
            Co-authored-by: Leo Messi <leo.messi@example.com>\n\
            Co-authored-by: Emi Martinez <emi.martinez@example.com>\n\n",
        ));

    Ok(())
}

mod helpers;

use assert_cmd::prelude::*;
use helpers::test_contexts::TestContextRepo;
use predicates::prelude::*;
use std::env;
use test_context::test_context;

#[test_context(TestContextRepo, skip_teardown)]
#[test]
fn test_prepare_commit_msg(ctx: TestContextRepo) -> Result<(), Box<dyn std::error::Error>> {
    ctx.git()
        .args(["config", "core.hooksPath", env!("CARGO_MANIFEST_DIR")])
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
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // git commit prints Co-authored-by trailers and an empty line to stderr
    ctx.git()
        .args(["commit", "--allow-empty", "--message", "test: hello world!"])
        .assert()
        .success()
        .stderr(predicate::str::diff(
            "Co-authored-by: Leo Messi <leo.messi@example.com>\nCo-authored-by: Emi Martinez <emi.martinez@example.com>\n\n",
        ));

    // the commit body has message with Co-authored-by trailers
    ctx.git()
        .args(["show", "--no-patch", "--format=%B"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "test: hello world!\n\nCo-authored-by: Leo Messi <leo.messi@example.com>\nCo-authored-by: Emi Martinez <emi.martinez@example.com>\n\n"
        ));

    Ok(())
}

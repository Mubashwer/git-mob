use crate::common::TestContext;
use assert_cmd::prelude::*;
use predicates::prelude::*;

pub mod common;

#[test]
fn test_add_coauthors() {
    let cx = TestContext::new();

    // adding 2 co-authors
    cx.git([
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

    cx.git([
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
    cx.git(["mob", "coauthor", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "lm Leo Messi <leo.messi@example.com>\nem Emi Martinez <emi.martinez@example.com>\n",
        ));
}

#[test]
fn test_delete_coauthor() {
    let cx = TestContext::new();

    // adding 2 co-authors
    cx.git([
        "mob",
        "coauthor",
        "--add",
        "lm",
        "Leo Messi",
        "leo.messi@example.com",
    ])
    .assert()
    .success();

    cx.git([
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
    cx.git(["mob", "coauthor", "--delete", "lm"])
        .assert()
        .success();

    // co-authors list excludes the deleted co-author
    cx.git(["mob", "coauthor", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "em Emi Martinez <emi.martinez@example.com>\n",
        ));
}

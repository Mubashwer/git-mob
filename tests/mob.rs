use crate::common::TestContext;
use assert_cmd::prelude::*;
use predicates::prelude::*;

pub mod common;

fn before_each() -> TestContext {
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

    cx
}

#[test]
fn test_mob_with_by_keys() {
    let cx = before_each();

    // mobbing with both of the co-authors
    cx.git(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // verifying mob list shows both of the co-authors
    cx.git(["mob", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));
}

#[test]
fn test_mob_clear() {
    let cx = before_each();

    // mobbing with both of the co-authors
    cx.git(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // clearing mob session
    cx.git(["mob", "--clear"]).assert().success();

    // verifying mob list is empty
    cx.git(["mob", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(""));
}

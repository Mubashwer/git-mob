use crate::common::TestContext;
use assert_cmd::prelude::*;
use predicates::prelude::predicate;

pub mod common;

#[test]
fn test_prepare_commit_msg() {
    let cx = TestContext::new();
    let repo = cx.repo();

    let configs = [
        ("core.hooksPath", env!("CARGO_MANIFEST_DIR")),
        ("user.name", "Robin Hood"),
        ("user.email", "robinhood@test.com"),
    ];
    for (option, value) in configs {
        repo.git(["config", option, value]).assert().success();
    }

    repo.git([
        "mob",
        "coauthor",
        "--add",
        "lj",
        "Little John",
        "littlejohn@test.com",
    ])
    .assert()
    .success();
    repo.git(["mob", "--with", "lj"]).assert().success();

    // git commit prints trailers and an empty line to stderr
    repo.git(["commit", "--allow-empty", "--message", "Royal subject"])
        .assert()
        .success()
        .stderr(predicate::str::diff(
            "Co-authored-by: Little John <littlejohn@test.com>\n\n",
        ));

    // the commit message has trailers
    repo.git(["show", "--no-patch", "--format=%B", "HEAD"])
        .assert()
        .success()
        .stdout("Royal subject\n\nCo-authored-by: Little John <littlejohn@test.com>\n\n");
}

mod helpers;

use assert_cmd::prelude::*;
use helpers::test_contexts::TestContextCli;
use predicates::prelude::*;
#[cfg(unix)]
use rexpect::session::spawn_command;
use std::error::Error;
use test_context::test_context;

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_with_by_keys(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    add_two_coauthors(&ctx)?;

    // mobbing with both of the co-authors
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // verifying mob session has both of the selected co-authors
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
fn test_mob_with_by_keys_no_coauthor_found(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "--with", "jk"])
        .assert()
        .success()
        .stderr(predicate::str::diff("No co-author found with key: jk\n"));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_with_multiselect_given_no_coauthors_added(
    ctx: TestContextCli,
) -> Result<(), Box<dyn Error>> {
    // running command to display mob session multiselect prompt
    ctx.git()
        .args(["mob", "--with"])
        .assert()
        .success()
        .stderr(predicate::str::diff(
            "No co-author(s) found. At least one co-author must be added\n",
        ));

    Ok(())
}

#[cfg(unix)]
#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_with_multiselect_select_none(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // given a mob session with 2 co-authors is active
    add_two_coauthors(&ctx)?;
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // running command to display mob session multiselect prompt
    let mut command = ctx.git();
    command.args(["mob", "--with"]);

    let mut session = spawn_command(command, Some(5000))?;
    session.exp_string("Select active co-author(s)")?;
    session.exp_string("Leo Messi <leo.messi@example.com>")?;
    session.exp_string("Emi Martinez <emi.martinez@example.com>")?;

    // pressing enter to end prompt without selecting any co-authors
    session.send_control('m')?;
    session.exp_string("Going solo!")?;
    session.process.wait()?;

    // verifying mob session is now empty
    ctx.git()
        .args(["mob", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff(""));

    Ok(())
}

#[cfg(unix)]
#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_with_multiselect_select_coauthor(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    add_two_coauthors(&ctx)?;

    // running command to display mob session multiselect prompt
    let mut command = ctx.git();
    command.args(["mob", "--with"]);

    let mut session = spawn_command(command, Some(5000))?;
    session.exp_string("Select active co-author(s)")?;
    session.exp_string("Leo Messi <leo.messi@example.com>")?;
    session.exp_string("Emi Martinez <emi.martinez@example.com>")?;

    // pressing space to select first co-author
    session.send(" ")?;
    session.flush()?;

    // pressing enter to end prompt
    session.send_control('m')?;
    session.process.wait()?;

    // verifying mob session has selected co-author
    ctx.git()
        .args(["mob", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff("Leo Messi <leo.messi@example.com>\n"));

    Ok(())
}

#[cfg(unix)]
#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_with_multiselect_escape(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // given a mob session  with 2 co-authors is active
    add_two_coauthors(&ctx)?;
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // running command to display mob session multiselect prompt
    let mut command = ctx.git();
    command.args(["mob", "--with"]);

    let mut session = spawn_command(command, Some(5000))?;
    session.exp_string("Select active co-author(s)")?;
    session.exp_string("Leo Messi <leo.messi@example.com>")?;
    session.exp_string("Emi Martinez <emi.martinez@example.com>")?;

    // pressing escape to cancel prompt
    session.send_control('[')?;
    session.exp_string("<canceled>")?;
    session.process.wait()?;

    // verifying mob list shows selected co-authors are unchanged
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
    add_two_coauthors(&ctx)?;

    // mobbing with both of the co-authors
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // clearing mob session
    ctx.git()
        .args(["mob", "--clear"])
        .assert()
        .success()
        .stdout(predicate::str::diff(""));

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
fn test_mob_clear_given_empty_mob_session(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // clearing mob session
    ctx.git()
        .args(["mob", "--clear"])
        .assert()
        .success()
        .stdout(predicate::str::diff(""));

    // verifying mob list is empty
    ctx.git()
        .args(["mob", "--list"])
        .assert()
        .success()
        .stdout("");

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_trailers(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    add_two_coauthors(&ctx)?;

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

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_trailers_given_empty_mob_session(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // verifying mob trailers is empty
    ctx.git()
        .args(["mob", "--trailers"])
        .assert()
        .success()
        .stdout("");

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_list_given_empty_mob_session(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // verifying mob list is empty
    ctx.git()
        .args(["mob", "--list"])
        .assert()
        .success()
        .stdout("");

    Ok(())
}

fn add_two_coauthors(ctx: &TestContextCli) -> Result<(), Box<dyn Error>> {
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

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
    add_two_team_members(&ctx)?;

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
fn test_mob_with_by_key_when_team_member_not_found(
    ctx: TestContextCli,
) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "--with", "jk"])
        .assert()
        .failure()
        .stderr(predicate::str::diff(
            "Error: \"No team member found with key: jk\"\n",
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_with_multiselect_given_no_team_members_added(
    ctx: TestContextCli,
) -> Result<(), Box<dyn Error>> {
    // running command to display mob session multiselect prompt
    ctx.git()
        .args(["mob", "--with"])
        .assert()
        .failure()
        .stderr(predicate::str::diff(
            "Error: \"No team member(s) found. At least one team member must be added\"\n",
        ));

    Ok(())
}

#[cfg(unix)]
#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_with_multiselect_when_select_none(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // given a mob session with 2 co-authors
    add_two_team_members(&ctx)?;
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
fn test_mob_with_multiselect_when_select_coauthor(
    ctx: TestContextCli,
) -> Result<(), Box<dyn Error>> {
    add_two_team_members(&ctx)?;

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
fn test_mob_with_multiselect_when_press_escape(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // given a mob session with 2 co-authors
    add_two_team_members(&ctx)?;
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
fn test_add_to_mob(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // adding a team member
    ctx.git()
        .args(["mob", "--add", "Leo Messi", "leo.messi@example.com"])
        .assert()
        .success()
        .stdout(predicate::str::diff("Leo Messi <leo.messi@example.com>\n"));

    // mob list shows the added team member
    ctx.git()
        .args(["mob", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::diff("Leo Messi <leo.messi@example.com>\n"));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_clear_mob(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    add_two_team_members(&ctx)?;

    // mobbing with both of the team members
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
fn test_clear_mob_given_empty_mob_session(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
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
fn test_mob_coauthor_trailers(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    add_two_team_members(&ctx)?;

    // mobbing with both of the team members
    ctx.git()
        .args(["mob", "--with", "lm", "em"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            "Leo Messi <leo.messi@example.com>\nEmi Martinez <emi.martinez@example.com>\n",
        ));

    // verifying mob co-author trailers show Co-authored-by trailers for both team members
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
fn test_mob_coauthor_trailers_given_empty_mob_session(
    ctx: TestContextCli,
) -> Result<(), Box<dyn Error>> {
    // verifying mob co-author trailers is empty
    ctx.git()
        .args(["mob", "--trailers"])
        .assert()
        .success()
        .stdout("");

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_list_mob_given_empty_mob_session(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    // verifying mob list is empty
    ctx.git()
        .args(["mob", "--list"])
        .assert()
        .success()
        .stdout("");

    Ok(())
}

fn add_two_team_members(ctx: &TestContextCli) -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

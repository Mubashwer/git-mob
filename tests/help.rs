mod helpers;

use assert_cmd::prelude::*;
use helpers::test_contexts::TestContextCli;
use mockall::predicate;
use std::error::Error;
use test_context::test_context;

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_help(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "help"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
r#"A CLI tool which can help users automatically add co-author(s) to git commits for pair/mob programming.

A user can attribute a git commit to more than one author by adding one or more Co-authored-by trailers to the commit's message. Co-authored commits are visible on GitHub. This CLI tool will helper users do the this automatically and also help them store and manage co-authors for pair/mob programming sessions.

Usage example:

git mob setup

git mob team-member --add lm "Leo Messi" leo.messi@example.com

git mob --with lm

Usage: git mob [COMMAND] [OPTIONS]

Commands:
  setup        Create global prepare-commit-msg githook which appends Co-authored-by trailers to commit message
  team-member  Add/delete/list team member(s) from team member repository
  help         Print this message or the help of the given subcommand(s)

Options:
  -w, --with [<COAUTHOR_KEY>...]
          Sets co-author(s) from team member(s) in the mob/pair programming session
          
          This will clear any existing co-author(s) in current session
          
          Usage example: git mob pair --with lm mj

  -a, --add <COAUTHOR_NAME> <COAUTHOR_EMAIL>
          Adds co-author to the mob/pair programming session (usually non-team member)
          
          Usage example: git mob --add "Leo Messi" leo.messi@example.com

  -c, --clear
          Clears the mob/pair programming session. Going solo!
          
          Usage example: git mob --clear

  -l, --list
          Lists co-author(s) in the mob/pair programming session
          
          Usage example: git mob --list

  -t, --trailers
          Lists Co-authored-by trailers in the mob/pair programming session
          
          Usage example: git mob --trailers

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
"#
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_mob_help_summary(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "-h"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
r#"A CLI tool which can help users automatically add co-author(s) to git commits for pair/mob programming

Usage: git mob [COMMAND] [OPTIONS]

Commands:
  setup        Create global prepare-commit-msg githook which appends Co-authored-by trailers to commit message
  team-member  Add/delete/list team member(s) from team member repository
  help         Print this message or the help of the given subcommand(s)

Options:
  -w, --with [<COAUTHOR_KEY>...]
          Sets co-author(s) from team member(s) in the mob/pair programming session
  -a, --add <COAUTHOR_NAME> <COAUTHOR_EMAIL>
          Adds co-author to the mob/pair programming session (usually non-team member)
  -c, --clear
          Clears the mob/pair programming session. Going solo!
  -l, --list
          Lists co-author(s) in the mob/pair programming session
  -t, --trailers
          Lists Co-authored-by trailers in the mob/pair programming session
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
"#
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_team_member_help(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "help", "team-member"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
r#"Add/delete/list team member(s) from team member repository

User must store team member(s) to team member repository by using keys before starting pair/mob programming session(s).

Usage: git mob team-member [OPTIONS]

Options:
  -a, --add <TEAM_MEMBER_KEY> <TEAM_MEMBER_NAME> <TEAM_MEMBER_EMAIL>
          Adds team member to team member repository
          
          Usage example: git mob team-member --add lm "Leo Messi" leo.messi@example.com

  -d, --delete <TEAM_MEMBER_KEY>
          Remove team member from team member repository
          
          Usage example: git mob team-member --delete lm

  -l, --list
          Lists team member(s) with keys(s) from team member repository
          
          Usage example: git mob team-member --list

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
"#
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_team_member_help_summary(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "team-member", "-h"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            r#"Add/delete/list team member(s) from team member repository

Usage: git mob team-member [OPTIONS]

Options:
  -a, --add <TEAM_MEMBER_KEY> <TEAM_MEMBER_NAME> <TEAM_MEMBER_EMAIL>
          Adds team member to team member repository
  -d, --delete <TEAM_MEMBER_KEY>
          Remove team member from team member repository
  -l, --list
          Lists team member(s) with keys(s) from team member repository
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
"#,
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_setup_help(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "help", "setup"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
r#"Create global prepare-commit-msg githook which appends Co-authored-by trailers to commit message

Usage: git mob setup [OPTIONS]

Options:
      --local
          Set up local prepare-commit-msg githook which invokes the global one
          
          Only need to be run for repo which overrides local hooks directory
          
          Usage example: git mob setup --local

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
"#
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_setup_help_summary(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "setup", "-h"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            r#"Create global prepare-commit-msg githook which appends Co-authored-by trailers to commit message

Usage: git mob setup [OPTIONS]

Options:
      --local    Set up local prepare-commit-msg githook which invokes the global one
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
"#,
        ));

    Ok(())
}

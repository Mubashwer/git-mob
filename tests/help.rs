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
r#"A CLI app which can help users automatically add co-author(s) to git commits for pair/mob programming.

A user can attribute a git commit to more than one author by adding one or more Co-authored-by trailers to the commit's message. Co-authored commits are visible on GitHub. This CLI app will helper users do the this automatically and also help them store and manage co-authors for pair/mob programming sessions.

Usage example:

git mob co-author --add lm "Leo Messi" leo.messi@example.com

git pair with

Usage: git mob [COMMAND] [OPTIONS]

Commands:
  coauthor  Add/delete/list co-author(s) from co-author repository
  help      Print this message or the help of the given subcommand(s)

Options:
  -w, --with [<COAUTHOR_KEY>...]
          Sets active co-author(s) for pair/mob programming session
          
          Usage example: git mob pair --with lm mj

  -c, --clear
          Clears mob/pair programming session. Going solo!
          
          Usage example: git mob co-author --list

  -l, --list
          Lists co-author(s) in current mob/pair programming session
          
          Usage example: git mob --list

  -t, --trailers
          Lists Co-authored-by trailers in current mob/pair programming session
          
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
r#"A CLI app which can help users automatically add co-author(s) to git commits for pair/mob programming

Usage: git mob [COMMAND] [OPTIONS]

Commands:
  coauthor  Add/delete/list co-author(s) from co-author repository
  help      Print this message or the help of the given subcommand(s)

Options:
  -w, --with [<COAUTHOR_KEY>...]  Sets active co-author(s) for pair/mob programming session
  -c, --clear                     Clears mob/pair programming session. Going solo!
  -l, --list                      Lists co-author(s) in current mob/pair programming session
  -t, --trailers                  Lists Co-authored-by trailers in current mob/pair programming session
  -h, --help                      Print help (see more with '--help')
  -V, --version                   Print version
"#
        ));

    Ok(())
}

#[test_context(TestContextCli, skip_teardown)]
#[test]
fn test_coauthor_help(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "help", "coauthor"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
r#"Add/delete/list co-author(s) from co-author repository

User must store co-author(s) to co-author repository by using keys before starting pair/mob programming session(s).

Usage: git mob coauthor [OPTIONS]

Options:
  -a, --add <COAUTHOR_KEY> <COAUTHOR_NAME> <COAUTHOR_EMAIL>
          Adds co-author to co-author repository
          
          Usage example: git mob co-author --add lm "Leo Messi" leo.messi@example.com

  -d, --delete <COAUTHOR_KEY>
          Remove co-author from co-author repository
          
          Usage example: git mob co-author --delete lm

  -l, --list
          Lists co-author(s) with keys(s) from co-author repository
          
          Usage example: git mob co-author --list

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
fn test_coauthor_help_summary(ctx: TestContextCli) -> Result<(), Box<dyn Error>> {
    ctx.git()
        .args(["mob", "coauthor", "-h"])
        .assert()
        .success()
        .stdout(predicate::str::diff(
            r#"Add/delete/list co-author(s) from co-author repository

Usage: git mob coauthor [OPTIONS]

Options:
  -a, --add <COAUTHOR_KEY> <COAUTHOR_NAME> <COAUTHOR_EMAIL>
          Adds co-author to co-author repository
  -d, --delete <COAUTHOR_KEY>
          Remove co-author from co-author repository
  -l, --list
          Lists co-author(s) with keys(s) from co-author repository
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
"#,
        ));

    Ok(())
}

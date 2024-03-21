use crate::coauthor_repo::CoauthorRepo;
use crate::commands::{coauthor::Coauthor, mob::Mob};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::io::Write;
use std::str;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about,
    bin_name = "git mob",
    override_usage = "git mob [COMMAND] [OPTIONS]"
)]
#[command(propagate_version = true)]
/// A CLI app which can help users automatically add co-author(s) to git commits
/// for pair/mob programming.
///
/// A user can attribute a git commit to more than one author by adding one or more
/// Co-authored-by trailers to the commit's message.
/// Co-authored commits are visible on GitHub.
/// This CLI app will helper users do the this automatically and also help them
/// store and manage co-authors for pair/mob programming sessions.
///
/// Usage example:
///
/// git mob co-author --add lm "Leo Messi" leo.messi@example.com
///
/// git pair with
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    mob: Mob,
}

#[derive(Subcommand)]
enum Commands {
    /// Add/delete/list co-author(s) from co-author repository
    ///
    /// User must store co-author(s) to co-author repository by using keys
    /// (usually initials) before starting pair/mob programming session(s).
    Coauthor(Coauthor),
}

pub fn run(
    coauthor_repo: &impl CoauthorRepo,
    out: &mut impl Write,
    err: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    run_inner(&cli, coauthor_repo, out, err)
}

fn run_inner(
    cli: &Cli,
    coauthor_repo: &impl CoauthorRepo,
    out: &mut impl Write,
    err: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    match &cli.command {
        None => cli.mob.handle(coauthor_repo, out, err)?,
        Some(Commands::Coauthor(coauthor)) => coauthor.handle(coauthor_repo, out, err)?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;
    use mockall::predicate;

    #[test]
    fn test_clear_mob_session() -> Result<(), Box<dyn Error>> {
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_clear_mob()
            .once()
            .returning(|| Ok(()));

        let cli = Cli {
            command: None,
            mob: Mob {
                with: None,
                clear: true,
                list: false,
            },
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        run_inner(&cli, &mock_coauthor_repo, &mut out, &mut err)?;

        Ok(())
    }

    #[test]
    fn test_delete_coauthor() -> Result<(), Box<dyn Error>> {
        let key = "lm";
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(Some("Leo Messi <leo.messi@example.com>".to_owned())));
        mock_coauthor_repo
            .expect_remove()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(()));

        let cli = Cli {
            command: Some(Commands::Coauthor(Coauthor {
                delete: Some(key.to_owned()),
                add: None,
                list: false,
            })),
            mob: Mob {
                with: None,
                clear: false,
                list: false,
            },
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        run_inner(&cli, &mock_coauthor_repo, &mut out, &mut err)?;

        Ok(())
    }
}

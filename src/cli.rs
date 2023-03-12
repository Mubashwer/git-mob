use crate::coauthor_repo::CoauthorRepo;
use crate::commands::{coauthor::Coauthor, mob::Mob};
use clap::{Parser, Subcommand};
use std::{io, str};

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
/// This app will do the above automatically and also help users store and manage
/// co-authors for pair/mob programming sessions.
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

pub fn run(coauthor_repo: &impl CoauthorRepo, writer: &mut impl io::Write) {
    let cli = Cli::parse();
    run_inner(&cli, coauthor_repo, writer);
}

fn run_inner(cli: &Cli, coauthor_repo: &impl CoauthorRepo, writer: &mut impl io::Write) {
    match &cli.command {
        None => cli.mob.handle(coauthor_repo, writer),
        Some(Commands::Coauthor(coauthor)) => coauthor.handle(coauthor_repo, writer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;
    use mockall::predicate;

    #[test]
    fn test_mob_clear_clears_mob() {
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_clear_mob()
            .once()
            .return_const({});

        let cli = Cli {
            command: None,
            mob: Mob {
                with: None,
                clear: true,
                list: false,
            },
        };

        let mut writer = Vec::new();
        run_inner(&cli, &mock_coauthor_repo, &mut writer);
    }

    #[test]
    fn test_coauthor_delete_removes_coauthor() {
        let key = "lm";
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_remove()
            .with(predicate::eq(key))
            .once()
            .return_const({});

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

        let mut writer = Vec::new();
        run_inner(&cli, &mock_coauthor_repo, &mut writer);
    }
}

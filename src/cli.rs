use crate::commands::{mob::Mob, setup::Setup, team_member::TeamMember};
use crate::mob_session_repo::MobSessionRepo;
use crate::team_member_repo::TeamMemberRepo;
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
/// A CLI tool which can help users automatically add co-author(s) to git commits
/// for pair/mob programming.
///
/// A user can attribute a git commit to more than one author by adding one or more
/// Co-authored-by trailers to the commit's message.
/// Co-authored commits are visible on GitHub.
/// This CLI tool will helper users do the this automatically and also help them
/// store and manage co-authors for pair/mob programming sessions.
///
/// Usage example:
///
/// git mob setup
///
/// git mob team-member --add lm "Leo Messi" leo.messi@example.com
///
/// git mob --with lm
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    mob: Mob,
}

#[derive(Subcommand)]
enum Commands {
    /// Create global prepare-commit-msg githook which appends Co-authored-by trailers to commit message
    Setup(Setup),
    /// Add/delete/list team member(s) from team member repository
    ///
    /// User must store team member(s) to team member repository by using keys
    /// before starting pair/mob programming session(s).
    #[clap(alias = "coauthor")] // alias for backward compatibility
    TeamMember(TeamMember),
}

pub fn run(
    team_member_repo: &impl TeamMemberRepo,
    mob_repo: &impl MobSessionRepo,
    out: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    run_inner(&cli, team_member_repo, mob_repo, out)
}

fn run_inner(
    cli: &Cli,
    team_member_repo: &impl TeamMemberRepo,
    mob_repo: &impl MobSessionRepo,
    out: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    match &cli.command {
        None => cli.mob.handle(team_member_repo, mob_repo, out)?,
        Some(Commands::Setup(setup)) => setup.handle(out)?,
        Some(Commands::TeamMember(team_member)) => team_member.handle(team_member_repo, out)?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::mob_session_repo::MockMobSessionRepo;
    use crate::team_member_repo::MockTeamMemberRepo;
    use mockall::predicate;

    #[test]
    fn test_clear_mob_session() -> Result<(), Box<dyn Error>> {
        let mock_team_member_repo = MockTeamMemberRepo::new();
        let mut mock_mob_repo = MockMobSessionRepo::new();
        mock_mob_repo.expect_clear().once().returning(|| Ok(()));

        let cli = Cli {
            command: None,
            mob: Mob {
                with: None,
                clear: true,
                list: false,
                trailers: false,
                add: None,
            },
        };

        let mut out = Vec::new();
        run_inner(&cli, &mock_team_member_repo, &mock_mob_repo, &mut out)?;

        Ok(())
    }

    #[test]
    fn test_delete_team_member() -> Result<(), Box<dyn Error>> {
        let key = "lm";
        let mut mock_team_member_repo = MockTeamMemberRepo::new();
        let mock_mob_repo = MockMobSessionRepo::new();
        mock_team_member_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(Some("Leo Messi <leo.messi@example.com>".to_owned())));
        mock_team_member_repo
            .expect_remove()
            .with(predicate::eq(key))
            .once()
            .returning(|_| Ok(()));

        let cli = Cli {
            command: Some(Commands::TeamMember(TeamMember {
                delete: Some(key.to_owned()),
                add: None,
                list: false,
            })),
            mob: Mob {
                with: None,
                clear: false,
                list: false,
                trailers: false,
                add: None,
            },
        };

        let mut out = Vec::new();
        run_inner(&cli, &mock_team_member_repo, &mock_mob_repo, &mut out)?;

        Ok(())
    }
}

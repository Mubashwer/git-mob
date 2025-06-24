use git_mob_tool::{
    cli,
    helpers::StdCommandRunner,
    repositories::{GitConfigMobRepo, GitConfigTeamMemberRepo},
    Result,
};
use std::io::stdout;

fn main() -> Result<()> {
    let team_member_repo = GitConfigTeamMemberRepo {
        command_runner: StdCommandRunner,
    };
    let mob_repo = GitConfigMobRepo {
        command_runner: StdCommandRunner,
    };
    let out = &mut stdout();
    cli::run(&team_member_repo, &mob_repo, out)?;
    Ok(())
}

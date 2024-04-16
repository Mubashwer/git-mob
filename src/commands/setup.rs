use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser;
use home::home_dir;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct Setup {
    /// Set up global prepare-commit-msg githook
    ///
    /// Usage example: git mob setup --global
    #[arg(short = 'g', long = "global")]
    pub(crate) global: bool,
    /// Set up local prepare-commit-msg githook which invokes the global one
    ///
    /// Only need to be run for repo which overrides local hooks directory
    ///
    /// Usage example: git mob setup --local
    #[arg(long = "local")]
    pub(crate) local: bool,
}

impl Setup {
    pub(crate) fn handle(&self, out: &mut impl Write) -> Result<(), Box<dyn Error>> {
        if self.global {
            self.handle_global(out)?;
        }

        if self.local {
            self.handle_local(out)?;
        }

        Ok(())
    }

    fn handle_global(&self, out: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let hooks_dir = match Self::get_hooks_dir("--global")? {
            Some(hooks_dir) => hooks_dir,
            None => {
                let new_hooks_dir = home_dir()
                    .ok_or("Failed to get home directory")?
                    .join(".git")
                    .join("hooks");

                Self::set_hooks_dir(out, &new_hooks_dir, "--global")?;

                new_hooks_dir
            }
        };

        let prepare_commit_msg_path = hooks_dir.join("prepare-commit-msg");

        if !hooks_dir.exists() {
            fs::create_dir_all(&hooks_dir)?;
        } else if prepare_commit_msg_path.exists() {
            Self::backup_prepare_commit_msg_hook(out, &prepare_commit_msg_path)?;
        }

        Self::create_prepare_commit_msg_hook(
            out,
            &prepare_commit_msg_path,
            include_str!("prepare-commit-msg"),
        )?;

        writeln!(out, "Setup complete")?;
        Ok(())
    }

    fn handle_local(&self, out: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let hooks_dir = match Self::get_hooks_dir("--local")? {
            Some(hooks_dir) => hooks_dir,
            None => return Err("Local githooks directory is not set".into()),
        };

        let prepare_commit_msg_path = hooks_dir.join("prepare-commit-msg");

        if !hooks_dir.exists() {
            fs::create_dir_all(&hooks_dir)?;
        } else if prepare_commit_msg_path.exists() {
            Self::backup_prepare_commit_msg_hook(out, &prepare_commit_msg_path)?;
        }

        Self::create_prepare_commit_msg_hook(
            out,
            &prepare_commit_msg_path,
            include_str!("prepare-commit-msg.local"),
        )?;

        writeln!(out, "Setup complete")?;
        Ok(())
    }

    fn get_hooks_dir(scope: &str) -> Result<Option<PathBuf>, Box<dyn Error>> {
        let output = Command::new("git")
            .args(["config", scope, "core.hooksPath"])
            .output()?;

        match output.status.success() {
            true => Ok(Some(PathBuf::from(
                String::from_utf8(output.stdout)?.trim(),
            ))),
            false => Ok(None),
        }
    }

    fn set_hooks_dir(out: &mut impl Write, path: &Path, scope: &str) -> Result<(), Box<dyn Error>> {
        let path_str = &path.to_string_lossy();
        let status = Command::new("git")
            .args(["config", scope, "core.hooksPath", path_str])
            .status()?;

        assert!(status.success());

        writeln!(out, "Set global githooks directory: {}", path_str)?;

        Ok(())
    }

    fn create_prepare_commit_msg_hook(
        out: &mut impl Write,
        path: &Path,
        contents: &str,
    ) -> Result<(), Box<dyn Error>> {
        fs::write(path, contents)?;

        #[cfg(unix)]
        {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, Permissions::from_mode(0o755))?; // Sets rwxr-xr-x permissions
        }

        writeln!(
            out,
            "Created new prepare-commit-msg githook: {}",
            &path.to_string_lossy()
        )?;

        Ok(())
    }

    fn backup_prepare_commit_msg_hook(
        out: &mut impl Write,
        path: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let backup_path = path.with_extension("bak");
        fs::rename(path, &backup_path)?;

        writeln!(
            out,
            "Backed up existing prepare-commit-msg githook: {}",
            &backup_path.to_string_lossy()
        )?;

        Ok(())
    }
}

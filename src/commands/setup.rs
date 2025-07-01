use crate::Result;
use clap::Parser;
use path_clean::PathClean;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser)]
pub(crate) struct Setup {
    /// Set up global prepare-commit-msg githook (deprecated, now default)
    ///
    /// Usage example: git mob setup --global
    #[arg(short = 'g', long = "global", hide = true)]
    // This option exists only for backward compatibility as global is now the default behavior
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
    pub(crate) fn handle(&self, out: &mut impl Write) -> Result<()> {
        if self.local {
            self.handle_local(out)?;
        } else {
            self.handle_global(out)?;
        }

        Ok(())
    }

    fn handle_global(&self, out: &mut impl Write) -> Result<()> {
        let hooks_dir = match Self::get_hooks_dir("--global")? {
            Some(hooks_dir) => hooks_dir,
            None => {
                let new_hooks_dir = env::home_dir()
                    .ok_or("Failed to get home directory")?
                    .join(".git")
                    .join("hooks")
                    .clean();

                Self::set_global_hooks_dir(out, &new_hooks_dir)?;

                new_hooks_dir
            }
        };

        let prepare_commit_msg_path = hooks_dir.join("prepare-commit-msg").clean();

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

    fn handle_local(&self, out: &mut impl Write) -> Result<()> {
        let hooks_dir = match Self::get_hooks_dir("--local")? {
            Some(hooks_dir) => hooks_dir,
            None => return Err("Local githooks directory is not set".into()),
        };

        let prepare_commit_msg_path = hooks_dir.join("prepare-commit-msg").clean();

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

    fn get_hooks_dir(scope: &str) -> Result<Option<PathBuf>> {
        let output = Command::new("git")
            .args(["config", scope, "core.hooksPath"])
            .output()?;

        if !output.status.success() {
            return Ok(None);
        }

        let hooks_dir = PathBuf::from(String::from_utf8(output.stdout)?.trim()).clean();
        if !hooks_dir.starts_with("~") {
            return Ok(Some(hooks_dir));
        }

        let mut expanded_hooks_dir = env::home_dir().ok_or("Failed to get home directory")?;
        expanded_hooks_dir.extend(hooks_dir.components().skip(1));
        Ok(Some(expanded_hooks_dir.clean()))
    }

    fn set_global_hooks_dir(out: &mut impl Write, path: &Path) -> Result<()> {
        let path_str = &path.to_string_lossy();
        let status = Command::new("git")
            .args(["config", "--global", "core.hooksPath", path_str])
            .status()?;

        if !status.success() {
            return Err(format!("Failed to set global githooks directory to {path_str}").into());
        }

        writeln!(out, "Set global githooks directory: {path_str}")?;

        Ok(())
    }

    fn create_prepare_commit_msg_hook(
        out: &mut impl Write,
        path: &Path,
        contents: &str,
    ) -> Result<()> {
        let clean_path = path.to_path_buf().clean();
        fs::write(&clean_path, contents)?;

        #[cfg(unix)]
        {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&clean_path, Permissions::from_mode(0o755))?; // Sets rwxr-xr-x permissions
        }

        writeln!(
            out,
            "Created new prepare-commit-msg githook: {}",
            &clean_path.to_string_lossy()
        )?;

        Ok(())
    }

    fn backup_prepare_commit_msg_hook(out: &mut impl Write, path: &Path) -> Result<()> {
        let clean_path = path.to_path_buf().clean();
        let backup_path = clean_path.with_extension("bak").clean();
        fs::rename(&clean_path, &backup_path)?;

        writeln!(
            out,
            "Backed up existing prepare-commit-msg githook: {}",
            &backup_path.to_string_lossy()
        )?;

        Ok(())
    }
}

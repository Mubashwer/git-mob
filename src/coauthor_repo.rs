use std::{
    error::Error,
    process::{Command, Output},
};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CoauthorRepo {
    fn list(&self, show_keys: bool) -> Result<Vec<String>, Box<dyn Error>>;
    fn list_mob(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>>;
    fn remove(&self, key: &str) -> Result<(), Box<dyn Error>>;
    fn add(&self, key: &str, coauthor: &str) -> Result<(), Box<dyn Error>>;
    fn add_to_mob(&self, coauthor: &str) -> Result<(), Box<dyn Error>>;
    fn clear_mob(&self) -> Result<(), Box<dyn Error>>;
}

pub struct GitConfigCoauthorRepo {}
impl GitConfigCoauthorRepo {
    const COAUTHORS_SECTION: &'static str = "coauthors";
    const COAUTHORS_MOB_SECTION: &'static str = "coauthors-mob";
    const COAUTHOR_MOB_KEY: &'static str = "entry";

    const EXIT_CODE_SUCCESS: i32 = 0;
    const EXIT_CODE_CONFIG_INVALID_KEY: i32 = 1;

    fn git_config_error<T>(output: &Output) -> Result<T, Box<dyn Error>> {
        match output.status.code() {
            Some(code) => Err(format!("Git config command exited with status code: {code}").into()),
            None => Err("Git config command terminated by signal".into()),
        }
    }
}

impl CoauthorRepo for GitConfigCoauthorRepo {
    fn list(&self, show_keys: bool) -> Result<Vec<String>, Box<dyn Error>> {
        let section = Self::COAUTHORS_SECTION;
        let search_regex = format!("^{section}\\.");

        let output = Command::new("git")
            .args(["config", "--global", "--get-regexp", &search_regex])
            .output()?;

        match output.status.code() {
            Some(Self::EXIT_CODE_SUCCESS) => String::from_utf8(output.stdout)?
                .lines()
                .map(|x| {
                    let delimiter = match show_keys {
                        true => format!("{section}."),
                        false => " ".to_owned(),
                    };
                    x.split_once(&delimiter)
                        .ok_or(format!("Failed to split string: '{x}'").into())
                        .map(|(_, coauthor)| coauthor.to_owned())
                })
                .collect(),

            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Ok(vec![]),
            _ => Self::git_config_error(&output),
        }
    }

    fn list_mob(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let full_key = format!("{}.{}", Self::COAUTHORS_MOB_SECTION, Self::COAUTHOR_MOB_KEY);

        let output = Command::new("git")
            .args(["config", "--global", "--get-all", &full_key])
            .output()?;

        match output.status.code() {
            Some(Self::EXIT_CODE_SUCCESS) => Ok(String::from_utf8(output.stdout)?
                .lines()
                .map(|x| x.into())
                .collect()),

            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Ok(vec![]),
            _ => Self::git_config_error(&output),
        }
    }

    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = Command::new("git")
            .args(["config", "--global", &full_key])
            .output()?;

        match output.status.code() {
            Some(Self::EXIT_CODE_SUCCESS) => {
                Ok(Some(String::from_utf8(output.stdout)?.trim().into()))
            }
            Some(Self::EXIT_CODE_CONFIG_INVALID_KEY) => Ok(None),
            _ => Self::git_config_error(&output),
        }
    }

    fn remove(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = Command::new("git")
            .args(["config", "--global", "--unset-all", &full_key])
            .output()?;

        match output.status.success() {
            true => Ok(()),
            false => Self::git_config_error(&output),
        }
    }

    fn add(&self, key: &str, coauthor: &str) -> Result<(), Box<dyn Error>> {
        let full_key = format!("{}.{key}", Self::COAUTHORS_SECTION);

        let output = Command::new("git")
            .args(["config", "--global", &full_key, coauthor])
            .output()?;

        match output.status.success() {
            true => Ok(()),
            false => Self::git_config_error(&output),
        }
    }

    fn add_to_mob(&self, coauthor: &str) -> Result<(), Box<dyn Error>> {
        let full_key = format!("{}.{}", Self::COAUTHORS_MOB_SECTION, Self::COAUTHOR_MOB_KEY);

        let output = Command::new("git")
            .args(["config", "--global", "--add", &full_key, coauthor])
            .output()?;

        match output.status.success() {
            true => Ok(()),
            false => Self::git_config_error(&output),
        }
    }

    fn clear_mob(&self) -> Result<(), Box<dyn Error>> {
        if self.list_mob()?.is_empty() {
            return Ok(());
        }

        let section = Self::COAUTHORS_MOB_SECTION.to_owned();

        let output = Command::new("git")
            .args(["config", "--global", "--remove-section", &section])
            .output()?;

        match output.status.success() {
            true => Ok(()),
            false => Self::git_config_error(&output),
        }
    }
}

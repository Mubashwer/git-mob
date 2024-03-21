use std::{error::Error, io, process::Command};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CoauthorRepo {
    fn list(&self, show_keys: bool) -> Result<Vec<String>, Box<dyn Error>>;
    fn list_mob(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>>;
    fn remove(&self, key: &str) -> Result<(), io::Error>;
    fn add(&self, key: &str, coauthor: &str) -> Result<(), io::Error>;
    fn add_to_mob(&self, coauthor: &str) -> Result<(), io::Error>;
    fn clear_mob(&self) -> Result<(), io::Error>;
}

pub struct GitConfigCoauthorRepo {}
impl GitConfigCoauthorRepo {
    const COAUTHORS_SECTION: &'static str = "coauthors";
    const COAUTHORS_MOB_SECTION: &'static str = "coauthors-mob";
    const COAUTHOR_MOB_KEY: &'static str = "entry";
}
type Gccr = GitConfigCoauthorRepo;

impl CoauthorRepo for GitConfigCoauthorRepo {
    fn list(&self, show_keys: bool) -> Result<Vec<String>, Box<dyn Error>> {
        let section = Gccr::COAUTHORS_SECTION;
        let search_regex = format!("^{section}\\.");

        let output = Command::new("git")
            .args(["config", "--global", "--get-regexp", &search_regex])
            .output()?;

        String::from_utf8(output.stdout)?
            .lines()
            .map(|x| {
                let delimiter = match show_keys {
                    true => format!("{section}."),
                    false => " ".to_owned(),
                };
                x.split_once(&delimiter)
                    .ok_or(format!("Failed to split string: '{}'", x).into())
                    .map(|(_, coauthor)| coauthor.to_owned())
            })
            .collect()
    }

    fn list_mob(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let full_key = format!("{}.{}", Gccr::COAUTHORS_MOB_SECTION, Gccr::COAUTHOR_MOB_KEY);

        let output = Command::new("git")
            .args(["config", "--global", "--get-all", &full_key])
            .output()?;

        Ok(String::from_utf8(output.stdout)?
            .lines()
            .map(|x| x.to_owned())
            .collect())
    }

    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let full_key = format!("{}.{key}", Gccr::COAUTHORS_SECTION);

        let output = Command::new("git")
            .args(["config", "--global", &full_key])
            .output()?;

        match output.status.success() {
            true => Ok(Some(String::from_utf8(output.stdout)?.trim().to_owned())),
            false => Ok(None),
        }
    }

    fn remove(&self, key: &str) -> Result<(), io::Error> {
        let full_key = format!("{}.{key}", Gccr::COAUTHORS_SECTION);

        let status = Command::new("git")
            .args(["config", "--global", "--unset-all", &full_key])
            .status()?;
        assert!(status.success());

        Ok(())
    }

    fn add(&self, key: &str, coauthor: &str) -> Result<(), io::Error> {
        let full_key = format!("{}.{key}", Gccr::COAUTHORS_SECTION);

        let status = Command::new("git")
            .args(["config", "--global", &full_key, coauthor])
            .status()?;
        assert!(status.success());

        Ok(())
    }

    fn add_to_mob(&self, coauthor: &str) -> Result<(), io::Error> {
        let full_key = format!("{}.{}", Gccr::COAUTHORS_MOB_SECTION, Gccr::COAUTHOR_MOB_KEY);

        let status = Command::new("git")
            .args(["config", "--global", "--add", &full_key, coauthor])
            .status()?;
        assert!(status.success());

        Ok(())
    }

    fn clear_mob(&self) -> Result<(), io::Error> {
        let section = Gccr::COAUTHORS_MOB_SECTION.to_owned();

        Command::new("git")
            .args(["config", "--global", "--remove-section", &section])
            .output()?;

        Ok(())
    }
}

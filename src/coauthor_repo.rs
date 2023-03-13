use std::process::Command;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CoauthorRepo {
    fn list(&self) -> Vec<String>;
    fn list_mob(&self) -> Vec<String>;
    fn get(&self, key: &str) -> Result<String, String>;
    fn remove(&self, key: &str);
    fn add(&self, key: &str, coauthor: &str);
    fn add_to_mob(&self, coauthor: &str);
    fn clear_mob(&self);
}

pub struct GitConfigCoauthorRepo {}
impl GitConfigCoauthorRepo {
    const COAUTHORS_SECTION_NAME: &str = "coauthors";
    const COAUTHORS_MOB_SECTION_NAME: &str = "coauthors-mob";
    const COAUTHOR_MOB_KEY: &str = "entry";
}
type GCCR = GitConfigCoauthorRepo;

impl CoauthorRepo for GitConfigCoauthorRepo {
    fn list(&self) -> Vec<String> {
        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--get-regexp")
            .arg(format!("^{}\\.", GCCR::COAUTHORS_SECTION_NAME))
            .output()
            .expect("failed to execute process");

        let options: Vec<String> = String::from_utf8(output.stdout)
            .expect("failed to convert stdout to string")
            .lines()
            .map(|x| {
                x.split_once(' ')
                    .expect("failed to split string")
                    .1
                    .to_string()
            })
            .collect();

        return options;
    }

    fn list_mob(&self) -> Vec<String> {
        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--get-all")
            .arg(format!(
                "{}.{}",
                GCCR::COAUTHORS_MOB_SECTION_NAME,
                GCCR::COAUTHOR_MOB_KEY
            ))
            .output()
            .expect("failed to execute process");

        let options: Vec<String> = String::from_utf8(output.stdout)
            .expect("failed to convert stdout to string")
            .lines()
            .map(|x| x.to_string())
            .collect();

        return options;
    }

    fn get(&self, key: &str) -> Result<String, String> {
        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg(format!("{}.{key}", GCCR::COAUTHORS_SECTION_NAME))
            .output()
            .expect("failed to execute process");

        return match output.status.success() {
            true => Ok(String::from_utf8(output.stdout)
                .expect("failed to convert stdout to string")
                .trim()
                .to_string()),
            false => {
                Err(String::from_utf8(output.stderr).expect("failed to convert stderr to string"))
            }
        };
    }

    fn remove(&self, key: &str) {
        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--unset-all")
            .arg(format!("{}.{key}", GCCR::COAUTHORS_SECTION_NAME))
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            eprintln!("No co-author found with key: {key}")
        }
    }

    fn add(&self, key: &str, coauthor: &str) {
        let status = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg(format!("{}.{key}", GCCR::COAUTHORS_SECTION_NAME))
            .arg(&coauthor)
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn add_to_mob(&self, coauthor: &str) {
        let status = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--add")
            .arg(format!(
                "{}.{}",
                GCCR::COAUTHORS_MOB_SECTION_NAME,
                GCCR::COAUTHOR_MOB_KEY
            ))
            .arg(&coauthor)
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn clear_mob(&self) {
        Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--remove-section")
            .arg(format!("{}", GCCR::COAUTHORS_MOB_SECTION_NAME))
            .output()
            .expect("failed to execute process");
    }
}

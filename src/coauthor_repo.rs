use std::process::Command;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CoauthorRepo {
    fn list(&self, show_keys: bool) -> Vec<String>;
    fn list_mob(&self) -> Vec<String>;
    fn get(&self, key: &str) -> Option<String>;
    fn remove(&self, key: &str);
    fn add(&self, key: &str, coauthor: &str);
    fn add_to_mob(&self, coauthor: &str);
    fn clear_mob(&self);
}

pub struct GitConfigCoauthorRepo {}
impl GitConfigCoauthorRepo {
    const COAUTHORS_SECTION: &str = "coauthors";
    const COAUTHORS_MOB_SECTION: &str = "coauthors-mob";
    const COAUTHOR_MOB_KEY: &str = "entry";
}
type GCCR = GitConfigCoauthorRepo;

impl CoauthorRepo for GitConfigCoauthorRepo {
    fn list(&self, show_keys: bool) -> Vec<String> {
        let prefix = GCCR::COAUTHORS_SECTION;
        let output = Command::new("git")
            .args(["config", "--global", "--get-regexp"])
            .arg(format!("^{prefix}\\."))
            .output()
            .expect("failed to execute process");

        String::from_utf8(output.stdout)
            .expect("failed to convert stdout to string")
            .lines()
            .map(|x| {
                let delimeter = match show_keys {
                    true => format!("{prefix}."),
                    false => " ".to_owned(),
                };
                x.split_once(&delimeter)
                    .expect("failed to split string")
                    .1
                    .to_string()
            })
            .collect()
    }

    fn list_mob(&self) -> Vec<String> {
        let output = Command::new("git")
            .args(["config", "--global", "--get-all"])
            .arg(format!(
                "{}.{}",
                GCCR::COAUTHORS_MOB_SECTION,
                GCCR::COAUTHOR_MOB_KEY
            ))
            .output()
            .expect("failed to execute process");

        String::from_utf8(output.stdout)
            .expect("failed to convert stdout to string")
            .lines()
            .map(|x| x.to_string())
            .collect()
    }

    fn get(&self, key: &str) -> Option<String> {
        let output = Command::new("git")
            .args(["config", "--global"])
            .arg(format!("{}.{key}", GCCR::COAUTHORS_SECTION))
            .output()
            .expect("failed to execute process");

        match output.status.success() {
            true => Some(
                String::from_utf8(output.stdout)
                    .expect("failed to convert stdout to string")
                    .trim()
                    .to_string(),
            ),
            false => None,
        }
    }

    fn remove(&self, key: &str) {
        let status = Command::new("git")
            .args(["config", "--global", "--unset-all"])
            .arg(format!("{}.{key}", GCCR::COAUTHORS_SECTION))
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn add(&self, key: &str, coauthor: &str) {
        let status = Command::new("git")
            .args(["config", "--global"])
            .arg(format!("{}.{key}", GCCR::COAUTHORS_SECTION))
            .arg(&coauthor)
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn add_to_mob(&self, coauthor: &str) {
        let status = Command::new("git")
            .args(["config", "--global", "--add"])
            .arg(format!(
                "{}.{}",
                GCCR::COAUTHORS_MOB_SECTION,
                GCCR::COAUTHOR_MOB_KEY
            ))
            .arg(&coauthor)
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn clear_mob(&self) {
        Command::new("git")
            .args(["config", "--global", "--remove-section"])
            .arg(format!("{}", GCCR::COAUTHORS_MOB_SECTION))
            .output()
            .expect("failed to execute process");
    }
}

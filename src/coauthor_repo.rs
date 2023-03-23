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
type Gccr = GitConfigCoauthorRepo;

impl CoauthorRepo for GitConfigCoauthorRepo {
    fn list(&self, show_keys: bool) -> Vec<String> {
        let section = Gccr::COAUTHORS_SECTION;
        let search_regex = format!("^{section}\\.");

        let output = Command::new("git")
            .args(["config", "--global", "--get-regexp", &search_regex])
            .output()
            .expect("failed to execute process");

        String::from_utf8(output.stdout)
            .expect("failed to convert stdout to string")
            .lines()
            .map(|x| {
                let delimeter = match show_keys {
                    true => format!("{section}."),
                    false => " ".to_owned(),
                };
                x.split_once(&delimeter)
                    .expect("failed to split string")
                    .1
                    .to_owned()
            })
            .collect()
    }

    fn list_mob(&self) -> Vec<String> {
        let full_key = format!("{}.{}", Gccr::COAUTHORS_MOB_SECTION, Gccr::COAUTHOR_MOB_KEY);

        let output = Command::new("git")
            .args(["config", "--global", "--get-all", &full_key])
            .output()
            .expect("failed to execute process");

        String::from_utf8(output.stdout)
            .expect("failed to convert stdout to string")
            .lines()
            .map(|x| x.to_owned())
            .collect()
    }

    fn get(&self, key: &str) -> Option<String> {
        let full_key = format!("{}.{key}", Gccr::COAUTHORS_SECTION);

        let output = Command::new("git")
            .args(["config", "--global", &full_key])
            .output()
            .expect("failed to execute process");

        match output.status.success() {
            true => Some(
                String::from_utf8(output.stdout)
                    .expect("failed to convert stdout to string")
                    .trim()
                    .to_owned(),
            ),
            false => None,
        }
    }

    fn remove(&self, key: &str) {
        let full_key = format!("{}.{key}", Gccr::COAUTHORS_SECTION);

        let status = Command::new("git")
            .args(["config", "--global", "--unset-all", &full_key])
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn add(&self, key: &str, coauthor: &str) {
        let full_key = format!("{}.{key}", Gccr::COAUTHORS_SECTION);

        let status = Command::new("git")
            .args(["config", "--global", &full_key, coauthor])
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn add_to_mob(&self, coauthor: &str) {
        let full_key = format!("{}.{}", Gccr::COAUTHORS_MOB_SECTION, Gccr::COAUTHOR_MOB_KEY);

        let status = Command::new("git")
            .args(["config", "--global", "--add", &full_key, coauthor])
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn clear_mob(&self) {
        let section = Gccr::COAUTHORS_MOB_SECTION.to_owned();

        Command::new("git")
            .args(["config", "--global", "--remove-section", &section])
            .output()
            .expect("failed to execute process");
    }
}

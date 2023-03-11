use std::process::Command;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CoauthorRepo {
    fn list(&self) -> Vec<String>;
    fn list_mob(&self) -> Vec<String>;
    fn get(&self, key: &str) -> String;
    fn remove(&self, key: &str);
    fn add_to_mob(&self, coauthor: &str);
    fn clear_mob(&self);
}

pub struct GitConfigCoauthorRepo {}
impl CoauthorRepo for GitConfigCoauthorRepo {
    fn list(&self) -> Vec<String> {
        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--get-regexp")
            .arg("^coauthors\\.")
            .output()
            .expect("failed to execute process");

        assert!(output.status.success());
        let options: Vec<String> = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|x| x.split_once(' ').unwrap().1.to_string())
            .collect();

        return options;
    }

    fn list_mob(&self) -> Vec<String> {
        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--get-all")
            .arg("coauthors-mob.entry")
            .output()
            .expect("failed to execute process");

        let options: Vec<String> = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|x| x.to_string())
            .collect();

        return options;
    }

    fn get(&self, key: &str) -> String {
        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg(format!("coauthors.{key}"))
            .output()
            .expect("failed to execute process");

        assert!(output.status.success());
        return String::from_utf8(output.stdout).unwrap().trim().to_string();
    }

    fn remove(&self, key: &str) {
        let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--unset-all")
            .arg(format!("coauthors.{key}"))
            .output()
            .expect("failed to execute process");

        assert!(output.status.success());
    }

    fn add_to_mob(&self, coauthor: &str) {
        let status = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--add")
            .arg("coauthors-mob.entry")
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
            .arg("coauthors-mob")
            .output()
            .expect("failed to execute process");
    }
}

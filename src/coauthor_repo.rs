use std::process::Command;

pub(crate) trait CoauthorRepo {
    fn get_all(&self) -> Vec<String>;
    fn get(&self, key: &str) -> String;
    fn activate(&self, coauthor: &str);
    fn deactivate_all(&self);
}

pub(crate) struct GitConfigCoauthorRepo {}
impl CoauthorRepo for GitConfigCoauthorRepo {
    fn get_all(&self) -> Vec<String> {
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

    fn activate(&self, coauthor: &str) {
        let status = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--add")
            .arg("coauthors-active.entry")
            .arg(&coauthor)
            .status()
            .expect("failed to execute process");
        assert!(status.success());
    }

    fn deactivate_all(&self) {
        Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--remove-section")
            .arg("coauthors-active")
            .output()
            .expect("failed to execute process");
    }
}

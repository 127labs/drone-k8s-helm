#[cfg(test)]
mod tests;

use std::process::Command;

use utils;
use config::Config;

trait Plugin {
    fn build_clean_command(&self) -> Command;
    fn build_upgrade_command(&self) -> Command;
}

impl Plugin for Config {
    fn build_clean_command(&self) -> Command {
        let kubectl_bin = utils::which("kubectl").expect("Kubernetes CLI to be installed");
        let mut command = Command::new(kubectl_bin.to_str().unwrap());

        command
            .arg("delete")
            .arg("jobs")
            .arg("-l")
            .arg(format!("release={}", self.release.as_str().unwrap()));
        command
    }

    fn build_upgrade_command(&self) -> Command {
        let helm_bin = utils::which("helm").expect("Helm to be installed");
        let mut command = Command::new(helm_bin.to_str().unwrap());

        command
            .arg("upgrade")
            .arg("-i")
            .arg(self.release.as_str().unwrap());

        for (key, value) in self.values.as_object().unwrap() {
            command
                .arg("--set")
                .arg(format!("{}={}", key, value.as_str().unwrap()).as_str());
        }

        command.arg(self.chart.as_str().unwrap());
        command
    }
}


pub fn upgrade(config: &Config) -> () {
    config
        .build_upgrade_command()
        .status()
        .expect("Failed to execute helm upgrade command");
}

pub fn clean(config: &Config) -> () {
    let clean_before_release = config.clean_before_release.as_bool().unwrap();

    if clean_before_release == true {
        config
            .build_clean_command()
            .status()
            .expect("Failed to delete jobs from master");
    }
}

pub fn init() -> () {
    let helm_bin = utils::which("helm").expect("Helm to be installed");

    Command::new(helm_bin.to_str().unwrap())
        .arg("init")
        .status()
        .expect("Failed to initialize helm");
}

#[cfg(test)]
mod tests;

use std::process::Command;

use utils;
use config::Config;

pub trait Plugin {
    fn build_clean_command(&self) -> Command;
    fn build_upgrade_command(&self) -> Command;
}

pub fn upgrade(config: &Config) -> () {
    config.build_upgrade_command().status().expect(
        "Failed to execute helm upgrade command",
    );
}

pub fn clean(config: &Config) -> () {
    let clean_before_release = config.clean_before_release.as_bool().unwrap();

    if clean_before_release == true {
        config.build_clean_command().status().expect(
            "Failed to delete jobs from master",
        );
    }
}

pub fn init() -> () {
    let helm_bin = utils::which("helm").expect("Helm to be installed");

    Command::new(helm_bin.to_str().unwrap())
        .arg("init")
        .status()
        .expect("Failed to initialize helm");
}

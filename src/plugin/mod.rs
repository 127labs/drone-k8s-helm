#[cfg(test)]
mod tests;

use std::process::Command;
use std::path::PathBuf;
use std::env;

use config::Config;

fn which(exe: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                            let full_path = dir.join(&exe);
                            if full_path.is_file() {
                                Some(full_path)
                            } else {
                                None
                            }
                        })
            .next()
    })
}

fn build_upgrade_command(config: &Config) -> Command {
    let helm_bin = which("helm").expect("Helm to be installed");
    let mut command = Command::new(helm_bin.to_str().unwrap());

    command
        .arg("upgrade")
        .arg("-i")
        .arg(config.release.as_str().unwrap());

    for (key, value) in config.values.as_object().unwrap() {
        command
            .arg("--set")
            .arg(format!("{}={}", key, value.as_str().unwrap()).as_str());
    }

    command.arg(config.chart.as_str().unwrap());
    command
}

fn build_clean_command(config: &Config) -> Command {
    let kubectl_bin = which("kubectl").expect("Kubernetes CLI to be installed");
    let mut command = Command::new(kubectl_bin.to_str().unwrap());

    command
        .arg("delete")
        .arg("jobs")
        .arg("-l")
        .arg(format!("release={}", config.release.as_str().unwrap()));
    command
}

pub fn upgrade(config: &Config) -> () {
    build_upgrade_command(config)
        .status()
        .expect("Failed to execute helm upgrade command");
}

pub fn clean(config: &Config) -> () {
    let clean_before_release = config.clean_before_release.as_bool().unwrap();

    if clean_before_release == true {
        build_clean_command(config)
            .status()
            .expect("Failed to delete jobs from master");
    }
}

pub fn init() -> () {
    let helm_bin = which("helm").expect("Helm to be installed");

    Command::new(helm_bin.to_str().unwrap())
        .arg("init")
        .status()
        .expect("Failed to initialize helm");
}

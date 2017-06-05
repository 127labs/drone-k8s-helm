use std::process::Command;
use std::path::PathBuf;
use std::env;

use config::Config;

fn which(exe: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}

pub fn upgrade(config: &Config) -> () {
    let helm_bin = which("helm").unwrap();
    let mut command = Command::new(helm_bin.to_str().unwrap());

    command.arg("upgrade").arg("-i").arg(config.release.as_ref().unwrap());

    for (key, value) in &config.values {
        command.arg("--set").arg(format!("{}={}", key, value).as_str());
    }

    command.arg(config.chart.as_ref().unwrap());

    command.status().expect("Failed to execute helm upgrade command");
}

pub fn init() -> () {
    let helm_bin = which("helm").unwrap();

    Command::new(helm_bin.to_str().unwrap())
        .arg("init")
        .status()
        .expect("Failed to initialize helm");
}

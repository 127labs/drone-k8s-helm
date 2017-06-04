use std::process::Command;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::env;
use std::collections::BTreeMap;

use config::Config;
use handlebars::Handlebars;

const TEMPLATE: &'static str = "\
apiVersion: v1
clusters:
- cluster:
    insecure-skip-tls-verify: {{ skip_tls }}
    server: {{ master }}
  name: helm
contexts:
- context:
    cluster: helm
    namespace: {{ namespace }}
    user: helm
  name: helm
current-context: helm
kind: Config
preferences: {}
users:
- name: helm
  user:
    token: {{ token }}\
";

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

fn create_config_file() -> File {
    let mut config_path = env::home_dir().expect("Failed to find home directory");

    config_path.push(".kube");

    fs::create_dir_all(config_path.as_path()).expect("Failed to create config directory");

    config_path.push("config");

    File::create(config_path).expect("Failed to create config file")
}

pub fn write_config(config: &Config) -> () {
    let mut handlebars = Handlebars::new();
    let mut assigns = BTreeMap::new();

    handlebars.register_template_string("config", TEMPLATE)
        .expect("Failed to register template");

    assigns.insert("master", &config.master);
    assigns.insert("namespace", &config.namespace);
    assigns.insert("skip_tls", &config.skip_tls);
    assigns.insert("token", &config.token);

    let rendered_config = handlebars.render("config", &assigns)
        .expect("Failed to render kube config");

    let mut buffer = create_config_file();

    buffer.write(&rendered_config.into_bytes()).expect("Failed to write config");
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

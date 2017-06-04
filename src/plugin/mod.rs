use std::process::Command;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;

use config::Config;
use tera::Context;

const HELM_BIN: &'static str = "/bin/helm";
const CONFIG_DIR: &'static str = "/root/.kube";
const CONFIG: &'static str = "config";

pub fn write_config(config: &Config) -> () {
    let tera = compile_templates!("./templates/**/*");

    let mut context = Context::new();

    context.add("master", &config.master);
    context.add("namespace", &config.namespace);
    context.add("skip_tls", &config.skip_tls);
    context.add("token", &config.token);

    let kube_config = tera.render("kube_config", &context).expect("Failed to render kube config");

    fs::create_dir_all(CONFIG_DIR).expect("Failed to create config directory");

    let mut buffer = File::create(Path::new(CONFIG_DIR).join(CONFIG))
        .expect("Failed to create config file");

    buffer.write(&kube_config.into_bytes()).expect("Failed to write config");
}

pub fn upgrade(config: &Config) -> () {
    let mut command = String::new();

    command.push_str(format!("{} upgrade -i {} ",
                             HELM_BIN,
                             config.release.as_ref().unwrap())
        .as_str());

    for (key, value) in &config.values {
        command.push_str(format!("--set {}={} ", key, value).as_str());
    }

    command.push_str(format!("{}", config.chart.as_ref().unwrap()).as_str());

    println!("{:?}",
             Command::new("sh")
                 .arg("-c")
                 .arg(command.as_str())
                 .output()
                 .expect("Failed to execute helm upgrade command"));
}

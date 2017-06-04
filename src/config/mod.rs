use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Error;
use std::process::Output;
use std::process::Command;
use std::path::Path;

use regex::Regex;
use tera::Context;

const VALUE_PREFIX: &'static str = "PLUGIN_SET_";
const HELM_BIN: &'static str = "/bin/helm";
const CONFIG_DIR: &'static str = "./root/.kube";
const CONFIG: &'static str = "config";

#[derive(Debug)]
pub struct Config {
    pub chart: Option<String>,
    pub master: Option<String>,
    pub namespace: Option<String>,
    pub release: Option<String>,
    pub skip_tls: Option<String>,
    pub token: Option<String>,
    pub values: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            chart: None,
            master: None,
            namespace: None,
            release: None,
            skip_tls: None,
            token: None,
            values: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) -> () {
        self.load();
        self.load_plugin_set();
        self.load_plugin_values();
        self.build_kube_config();
    }

    pub fn command_upgrade(&self) -> Result<Output, Error> {
        let mut command = String::new();

        command.push_str(format!("{} upgrade -i {} ", HELM_BIN, self.release.as_ref().unwrap()).as_str());

        for (key, value) in &self.values {
            command.push_str(format!("--set {}={} ", key, value).as_str());
        }

        command.push_str(format!("{}", self.chart.as_ref().unwrap()).as_str());

        println!("{}", command);

        Command::new("sh")
            .arg("-c")
            .arg(command.as_str())
            .output()
    }

    pub fn build_kube_config(&self) -> () {
        let tera = compile_templates!("templates/**/*");

        let mut context = Context::new();

        context.add("chart", &self.chart);
        context.add("master", &self.master);
        context.add("namespace", &self.namespace);
        context.add("release", &self.release);
        context.add("skip_tls", &self.skip_tls);
        context.add("token", &self.token);

        let config = tera.render("kube_config", &context).unwrap();

        fs::create_dir_all(CONFIG_DIR);

        let mut buffer = File::create(Path::new(CONFIG_DIR).join(CONFIG)).unwrap();

        buffer.write(&config.into_bytes());
    }

    fn load(&mut self) -> () {
        self.chart = Some(env::var("PLUGIN_CHART").unwrap());
        self.master = Some(env::var("PLUGIN_MASTER").unwrap());
        self.namespace = Some(env::var("PLUGIN_NAMESPACE").unwrap());
        self.release = Some(env::var("PLUGIN_RELEASE").unwrap());
        self.skip_tls = Some(env::var("PLUGIN_SKIP_TLS").unwrap());
        self.token = Some(env::var("PLUGIN_TOKEN").unwrap());
    }

    fn load_plugin_set(&mut self) -> () {
        for (key, val) in env::vars().filter(|&(ref key, _)| key.starts_with(VALUE_PREFIX)) {
            self.values.insert(key.replace(VALUE_PREFIX, ""), val);
        }
    }

    fn load_plugin_values(&mut self) -> () {
        let re = Regex::new(r"^(\w+)=(.+)$").unwrap();
        let values = env::var("PLUGIN_VALUES").unwrap();

        for key_value_pair in values.split(",") {
            let captures = re.captures(key_value_pair).unwrap();
            let key = captures.get(1).unwrap().as_str().to_string();
            let value = captures.get(2).unwrap().as_str().to_string();
            self.values.insert(key, value);
        }
    }
}

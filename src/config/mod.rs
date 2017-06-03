use std::env;
use regex::Regex;
use std::collections::HashMap;

const VALUE_PREFIX: &'static str = "PLUGIN_SET_";

#[derive(Debug)]
pub struct Config {
    pub chart: Option<String>,
    pub master: Option<String>,
    pub release: Option<String>,
    pub tls: Option<String>,
    pub token: Option<String>,
    pub values: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            chart: None,
            master: None,
            release: None,
            tls: None,
            token: None,
            values: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) -> () {
        self.load();
        self.load_plugin_set();
        self.load_plugin_values();
    }

    fn load(&mut self) -> () {
        self.chart = Some(env::var("PLUGIN_CHART").unwrap());
        self.master = Some(env::var("PLUGIN_MASTER").unwrap());
        self.release = Some(env::var("PLUGIN_RELEASE").unwrap());
        self.tls = Some(env::var("PLUGIN_TLS").unwrap());
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

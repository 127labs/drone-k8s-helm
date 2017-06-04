use std::collections::HashMap;
use std::env;

use regex::Regex;

const VALUE_PREFIX: &'static str = "PLUGIN_SET_";

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
    fn default() -> Config {
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

    pub fn new() -> Config {
        let mut config = Config::default();

        config.load();
        config.load_plugin_set();
        config.load_plugin_values();

        config
    }

    fn load(&mut self) -> () {
        self.chart = Some(env::var("PLUGIN_CHART").expect("PLUGIN_CHART env must be set"));
        self.master = Some(env::var("PLUGIN_MASTER").expect("PLUGIN_MASTER env must be set"));
        self.namespace = Some(env::var("PLUGIN_NAMESPACE").unwrap_or("default".to_string()));
        self.release = Some(env::var("PLUGIN_RELEASE").expect("PLUGIN_RELEASE env must be set"));
        self.skip_tls = Some(env::var("PLUGIN_SKIP_TLS").unwrap_or("false".to_string()));
        self.token = Some(env::var("PLUGIN_TOKEN").expect("PLUGIN_TOKEN env must be set"));
    }

    fn load_plugin_set(&mut self) -> () {
        for (key, val) in env::vars().filter(|&(ref key, _)| key.starts_with(VALUE_PREFIX)) {
            self.values.insert(key.replace(VALUE_PREFIX, ""), val);
        }
    }

    fn load_plugin_values(&mut self) -> () {
        let re = Regex::new(r"^(\w+)=(.+)$").unwrap();
        let values = env::var("PLUGIN_VALUES").unwrap_or(String::new());

        for key_val in values.split(",") {
            match key_val {
                "" => break,
                kv => {
                    let captures = re.captures(kv).unwrap();
                    let key = captures.get(1).unwrap().as_str().to_string();
                    let value = captures.get(2).unwrap().as_str().to_string();
                    self.values.insert(key, value);
                }
            }
        }
    }
}

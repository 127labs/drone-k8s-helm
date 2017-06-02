use std::env;
use regex::Regex;
use std::collections::HashMap;

const VALUE_PREFIX: &'static str = "PLUGIN_SET_";

pub struct Config {
    pub values: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Config {
        Config { values: HashMap::new() }
    }

    pub fn load_values(&mut self) {
        let re = Regex::new(r"^(\w+)=(.+)$").unwrap();

        let values = env::var("PLUGIN_VALUES").unwrap();

        for key_value_pair in values.split(",") {
          let captures = re.captures(key_value_pair).unwrap();
          let key = captures.get(1).unwrap().as_str().to_string();
          let value = captures.get(2).unwrap().as_str().to_string();
          self.values.insert(key, value);
        }

        for (key, val) in env::vars().filter(|&(ref key, _)| key.starts_with(VALUE_PREFIX)) {
          self.values.insert(key.replace(VALUE_PREFIX, ""), val);
        }
    }
}

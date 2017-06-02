use std::env;
// use regex::Regex;

const VALUE_PREFIX: &'static str = "PLUGIN_SET_";

pub struct Config {
    pub values: Vec<Value>,
}

impl Config {
    pub fn new() -> Config {
        Config { values: Vec::new() }
    }

    // pub fn put_value(&mut self, value: Value) {
    //     self.values.push(value);
    // }

    pub fn load_values(&mut self) {
        // let re = Regex::new(r"^(\w+)=(.+)$").unwrap();

        // let default_values = match env::var("PLUGIN_VALUES") {
        //     Ok(vals) => vals.split(",")
        //                     .map(|key_value_pair| {
        //                         let captures = re.captures(key_value_pair).unwrap();
        //                         Value::new(captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str())
        //                     })
        //                     .collect(),
        //     _ => Vec::new(),
        // };


        self.values = env::vars()
            .filter(|&(ref key, _)| key.starts_with(VALUE_PREFIX))
            .map(|(key, val)| Value::new(&key[VALUE_PREFIX.len()..], &val))
            .collect()
            // .extend(default_values.iter());
    }
}

pub struct Value {
    pub key: String,
    pub value: String,
}

impl Value {
    pub fn new(key: &str, value: &str) -> Value {
        Value {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

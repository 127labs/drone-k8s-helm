use std::env;

pub struct Config {
    pub values: Vec<Value>,
}

impl Config {
    pub fn new() -> Config {
        Config { values: Vec::new() }
    }

    pub fn put_value(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn load_values(&mut self) {
        self.values = env::vars()
            .filter(|&(ref key, _)| key.starts_with("PLUGIN_SET"))
            .map(|(key, val)| Value::new(&key, &val))
            .collect();
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

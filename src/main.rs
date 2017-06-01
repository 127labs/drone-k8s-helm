use std::env;

struct Config {
    values: Vec<Value>,
}

struct Value {
    key: String,
    value: String,
}

impl Value {
    fn new(key: &str, value: &str) -> Value {
        Value {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl Config {
    fn new() -> Config {
        Config { values: Vec::new() }
    }

    fn put_value(&mut self, value: Value) {
        self.values.push(value);
    }

    fn load_values(&mut self) {
        self.values = env::vars()
            .filter(|&(ref key, _)| key.starts_with("PLUGIN_SET"))
            .map(|(key, val)| Value::new(&key, &val))
            .collect();
    }
}

fn main() {
    let mut config = Config::new();

    config.load_values();

    for value in config.values {
        println!("{}={}", value.key, value.value);
    }
}

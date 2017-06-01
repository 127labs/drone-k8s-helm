use std::env;

struct Config {
    values: Vec<Value>,
}

struct Value {
    key: String,
    value: String
}

impl Value {
    fn new(key: &str, value: &str) -> Value {
        Value { key: key.to_string(), value: value.to_string() }
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
        let mut values = env.vars.filter_map(|var| )
    }
}

fn main() {
    let mut config = Config::new();

    let value = Value::new("HOST", "ilinked.asia");

    config.put_value(value);

    for value in config.values {
        println!("{}={}", value.key, value.value);
    }
}

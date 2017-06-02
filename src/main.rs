extern crate regex;

mod config;

use config::Config;

fn main() {
    let mut config = Config::new();

    config.load_values();

    for value in config.values {
        println!("{}={}", value.key, value.value);
    }
}

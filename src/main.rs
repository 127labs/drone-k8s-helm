extern crate regex;

mod config;

use config::Config;

fn main() {
    let mut config = Config::new();

    config.initialize();

    println!("{:?}", config);
}

extern crate regex;
#[macro_use(compile_templates)]
extern crate tera;

mod config;

use config::Config;

fn main() {
    let mut config = Config::new();

    config.initialize();
    println!("{:?}", config.command_upgrade().unwrap());
}

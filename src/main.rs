extern crate regex;
extern crate handlebars;

mod config;
mod plugin;

use config::Config;

fn main() -> () {
    let config = Config::new();

    plugin::init();
    plugin::upgrade(&config);
}

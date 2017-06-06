extern crate regex;
extern crate handlebars;
extern crate serde_json;

mod config;
mod plugin;

use config::Config;

fn main() -> () {
    let config = Config::new();

    plugin::init();
    plugin::clean(&config);
    plugin::upgrade(&config);
}

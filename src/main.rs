extern crate regex;
extern crate handlebars;

mod config;
mod plugin;

use config::Config;

fn main() -> () {
    let config = Config::new();

    plugin::write_config(&config);
    plugin::upgrade(&config);
}

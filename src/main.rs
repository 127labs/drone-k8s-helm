extern crate regex;
#[macro_use(compile_templates)]
extern crate tera;

mod config;
mod plugin;

use config::Config;

fn main() {
    let config = Config::new();

    plugin::write_config(&config);
    plugin::upgrade(&config);
}

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use handlebars::Handlebars;
use std::collections::BTreeMap;

use regex::Regex;

const VALUE_PREFIX: &'static str = "PLUGIN_SET_";
const TEMPLATE: &'static str = "\
apiVersion: v1
clusters:
- cluster:
    insecure-skip-tls-verify: {{ skip_tls }}
    server: {{ master }}
  name: helm
contexts:
- context:
    cluster: helm
    namespace: {{ namespace }}
    user: helm
  name: helm
current-context: helm
kind: Config
preferences: {}
users:
- name: helm
  user:
    token: {{ token }}\
";

#[derive(Debug)]
pub struct Config {
    pub chart: Option<String>,
    pub master: Option<String>,
    pub namespace: Option<String>,
    pub release: Option<String>,
    pub skip_tls: Option<String>,
    pub token: Option<String>,
    pub values: HashMap<String, String>,
    pub file: Option<File>,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config::default();

        config.load();
        config.load_plugin_set();
        config.load_plugin_values();
        config.create_file();
        config.write_file();

        config
    }

    fn default() -> Config {
        Config {
            chart: None,
            master: None,
            namespace: None,
            release: None,
            skip_tls: None,
            token: None,
            file: None,
            values: HashMap::new(),
        }
    }

    fn write_file(& self) -> () {
        let mut handlebars = Handlebars::new();
        let mut assigns = BTreeMap::new();

        handlebars.register_template_string("config", TEMPLATE)
            .expect("Failed to register template");

        assigns.insert("master", &self.master);
        assigns.insert("namespace", &self.namespace);
        assigns.insert("skip_tls", &self.skip_tls);
        assigns.insert("token", &self.token);

        let rendered_config = handlebars.render("config", &assigns)
            .expect("Failed to render kube config");

        self.file.as_ref()
            .expect("File is not set")
            .write(&rendered_config.into_bytes())
            .expect("Failed to write config");
    }

    fn create_file(&mut self) -> () {
        let mut config_path = env::home_dir().expect("Failed to find home directory");

        config_path.push(".kube");

        fs::create_dir_all(config_path.as_path()).expect("Failed to create config directory");

        config_path.push("config");

        self.file = Some(File::create(config_path).expect("Failed to create config file"));
    }

    fn load(&mut self) -> () {
        self.chart = Some(env::var("PLUGIN_CHART").expect("PLUGIN_CHART env must be set"));
        self.master = Some(env::var("PLUGIN_MASTER").expect("PLUGIN_MASTER env must be set"));
        self.namespace = Some(env::var("PLUGIN_NAMESPACE").unwrap_or("default".to_string()));
        self.release = Some(env::var("PLUGIN_RELEASE").expect("PLUGIN_RELEASE env must be set"));
        self.skip_tls = Some(env::var("PLUGIN_SKIP_TLS").unwrap_or("false".to_string()));
        self.token = Some(env::var("PLUGIN_TOKEN").expect("PLUGIN_TOKEN env must be set"));
    }

    fn load_plugin_set(&mut self) -> () {
        for (key, val) in env::vars().filter(|&(ref key, _)| key.starts_with(VALUE_PREFIX)) {
            self.values.insert(key.replace(VALUE_PREFIX, ""), val);
        }
    }

    fn load_plugin_values(&mut self) -> () {
        let re = Regex::new(r"^(\w+)=(.+)$").unwrap();
        let values = env::var("PLUGIN_VALUES").unwrap_or(String::new());

        for key_val in values.split(",") {
            match key_val {
                "" => break,
                kv => {
                    let captures = re.captures(kv).unwrap();
                    let key = captures.get(1).unwrap().as_str().to_string();
                    let value = captures.get(2).unwrap().as_str().to_string();
                    self.values.insert(key, value);
                }
            }
        }
    }
}

use super::*;
use serde_json::Value::Bool;

pub fn mock_helm_env_vars() {
    env::set_var("HELM_CHART", "stable/ghost");
    env::set_var("HELM_MASTER", "https://127.0.0.1:8001");
    env::set_var("HELM_NAMESPACE", "default");
    env::set_var("HELM_RELEASE", "blog");
    env::set_var("HELM_SKIP_TLS", "true");
    env::set_var("HELM_TOKEN", "some-secure-token-string");
    env::set_var("HELM_CLEAN_BEFORE_RELEASE", "true");
}

pub fn mock_helm_values_env_vars() {
    let json =
        r#"{"dokuwikiEmail":"{{DOKUWIKI_EMAIL}}","dokuwikiPassword":"{{DOKUWIKI_PASSWORD}}"}"#;
    env::set_var("HELM_VALUES", json);
    env::set_var("DOKUWIKI_EMAIL", "john.doe@127labs.com");
    env::set_var("DOKUWIKI_PASSWORD", "saltysea");
}

#[test]
fn load_populates_config_struct() {
    mock_helm_env_vars();

    let mut config = Config::default();
    config.load();

    assert_eq!(config.chart, "stable/ghost");
    assert_eq!(config.master, "https://127.0.0.1:8001");
    assert_eq!(config.namespace, "default");
    assert_eq!(config.release, "blog");
    assert_eq!(config.skip_tls, Bool(true));
    assert_eq!(config.token, "some-secure-token-string");
    assert_eq!(config.clean_before_release, Bool(true));
}

#[test]
fn parse_values_populates_values_field() {
    mock_helm_values_env_vars();

    let regex = Regex::new(r"^\{\{(\w+)\}\}$").unwrap();

    let mut config = Config::default();
    config.parse_values();

    for (_, value) in config.values.as_object().unwrap() {
        assert_eq!(regex.is_match(&value.as_str().unwrap().to_string()), false);
    }
}


#[test]
fn render_kube_config_template() {
    mock_helm_env_vars();
    mock_helm_values_env_vars();

    let mut config = Config::default();
    config.load();
    config.parse_values();

    assert_eq!(
        config.render_file(),
        "\
apiVersion: v1
clusters:
- cluster:
    insecure-skip-tls-verify: true
    server: https://127.0.0.1:8001
  name: helm
contexts:
- context:
    cluster: helm
    namespace: default
    user: helm
  name: helm
current-context: helm
kind: Config
preferences: {}
users:
- name: helm
  user:
    token: some-secure-token-string\
"
    )

}

use std::{fs, str::FromStr};

use toml::Value;

/// will load the config from the current working directory
pub fn load_config() -> Value {
    let s = fs::read_to_string("config.toml").unwrap();
    let mut cfg = s.parse::<Value>().unwrap();
    if cfg["token"].as_str() == Some("") {
        let key = cfg["token_env_variable"].as_str().unwrap();
        cfg["token"] = Value::from_str(&dotenv::var(key).unwrap()).unwrap();
    }
    if cfg["secret"].as_str() == Some("") {
        let key = cfg["secret_env_variable"].as_str().unwrap();
        cfg["secret"] = Value::from_str(&dotenv::var(key).unwrap()).unwrap();
    }
    cfg
}

/// function to verify that the config.toml file does not contain any errors
pub fn verify_config(_cfg: &Value) {
    // todo
    // panic!()
}

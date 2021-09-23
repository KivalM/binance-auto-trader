use crate::error::Error;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub time_interval: u32,
    pub num_intervals: u32,
    pub base_pair: String,
    pub api_token: String,
    pub api_secret: String,
    pub token_env_variable: String,
    pub secret_env_variable: String,
    pub discord_token: String,
    pub channel_ids: Vec<u64>,
    pub up: f64,
    pub down: f64,
    pub amount: f64,
    pub take_profit: f64,
    pub stop_loss: f64,
    pub buy_new_tokens: bool,
    pub new_token_amount: f64,
    pub tokens: Vec<Token>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Token {
    pub symbol: String,
    pub token: String,
    pub base: String,
    pub perc_up: f64,
    pub time_up: u32,
    pub perc_down: f64,
    pub time_down: u32,
    pub ratio: f64,
}

pub fn read_config() -> Result<String, Error> {
    let s = fs::read_to_string("Config.toml");
    match s {
        Ok(values) => Ok(values),
        Err(_) => Err(Error {
            code: 1,
            message: "Failed to read file".to_string(),
        }),
    }
}

pub fn load_config() -> Result<Config, Error> {
    let text = read_config()?;
    let z = toml::from_str(&text);
    match z {
        Ok(val) => {
            let mut cfg: Config = val;
            if cfg.api_token.is_empty() {
                match dotenv::var(cfg.token_env_variable.clone()) {
                    Ok(ans) => cfg.api_token = ans,
                    Err(e) => panic!("{}", e.to_string()),
                }
            }
            if cfg.api_secret.is_empty() {
                match dotenv::var(cfg.secret_env_variable.clone()) {
                    Ok(ans) => cfg.api_secret = ans,
                    Err(e) => panic!("{}", e.to_string()),
                }
            }

            Ok(cfg)
        }
        Err(e) => Err(Error {
            code: 2,
            message: e.to_string(),
        }),
    }
}

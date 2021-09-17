use std::{collections::HashMap, convert::TryInto, thread::sleep, time::Duration};

use binance::{account::Account, api::Binance, market::Market};
use queues::{IsQueue, Queue};
use toml::Value;

use crate::app::{
    account::{check_new, check_owned},
    notify::notify,
};

use self::{
    config::{load_config, verify_config},
    error::Error,
    watch::check_watched_symbols,
};

pub mod account;
pub mod buy;
pub mod config;
pub mod error;
pub mod notify;
pub mod sell;
pub mod watch;

/// will initialize variables
pub fn start() {
    let cfg = load_config();
    verify_config(&cfg);

    notify("Starting".to_string(), &cfg);

    let token: &str = cfg["token"].as_str().unwrap();
    let secret = cfg["secret"].as_str().unwrap();
    let market: Market = Binance::new(None, None);

    let mut owned: HashMap<String, f64> = HashMap::new();
    let mut watched: Vec<String> = Vec::new();

    let table = cfg["tokens"].as_table().unwrap();
    for i in table.keys() {
        let u = &table[i];
        watched.push(u["symbol"].as_str().unwrap().try_into().unwrap());
    }

    let mut new;
    let mut list: Queue<HashMap<String, f64>> = Queue::new();

    for _ in 0..cfg["num_intervals"].as_integer().unwrap() {
        new = account::get_prices(&market).unwrap_or_default();
        let _ = list.add(new.clone());
        sleep(Duration::from_secs(
            cfg["time_interval"]
                .as_integer()
                .unwrap()
                .try_into()
                .unwrap(),
        ));
    }
    let sleeptime = Duration::from_secs(
        cfg["time_interval"]
            .as_integer()
            .unwrap()
            .try_into()
            .unwrap(),
    );
    loop {
        let res = mainloop(token, secret, &cfg, &mut owned, &mut list, &watched);
        match res {
            Ok(own) => {
                owned = own;
                sleep(sleeptime);
            }
            Err(e) => {
                println!("Error: {}", e.message);
                continue;
            }
        }
    }
}

/// main loop with all of the major logics

pub fn mainloop(
    token: &str,
    secret: &str,
    cfg: &Value,
    owned: &mut HashMap<String, f64>,
    list: &mut Queue<HashMap<String, f64>>,
    watched: &Vec<String>,
) -> Result<HashMap<String, f64>, Error> {
    let market: Market = Binance::new(None, None);
    let account: Account = Binance::new(Some(token.to_string()), Some(secret.to_string()));

    let new = account::get_prices(&market)?;
    let _ = list.add(new.clone());
    let old = list.remove().unwrap();
    check_watched_symbols(&market, &account, cfg, &new)?;
    check_new(
        &account,
        &old,
        &new,
        owned,
        cfg["up"].as_float().unwrap(),
        cfg["fiat"].as_str().unwrap(),
        cfg,
    )?;

    let owned_new = check_owned(owned, &new, cfg, &old, watched, &account)?;
    Ok(owned_new)
}

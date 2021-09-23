use std::{thread::sleep, time::Duration};

use account::get_prices;
use binance::api::Binance;

use error::Error;
use watch::check_watched_symbols;

use crate::general::ApiInfo;

mod account;
mod buy;
mod config;
mod error;
mod general;
mod market;
mod notify;
mod sell;

mod watch;
fn main() {
    println!("Hello, world!");
    let conf = config::load_config().unwrap();
    let cfg = ApiInfo {
        account: Binance::new(Some(conf.api_token.clone()), Some(conf.api_secret.clone())),

        general: Binance::new(None, None),
        config: conf,
        market: Binance::new(None, None),
    };
    loop {
        match watch(cfg.clone()) {
            Ok(_) => sleep(Duration::from_secs(cfg.config.time_interval.into())),
            Err(e) => {
                println!("{}", e.message);
                if e.message == "Received response: 429" {
                    sleep(Duration::from_secs(30))
                }
            }
        }
    }
}

fn watch(mut cfg: ApiInfo) -> Result<(), Error> {
    cfg.account = Binance::new(
        Some(cfg.config.api_token.clone()),
        Some(cfg.config.api_secret.clone()),
    );
    cfg.general = Binance::new(None, None);
    cfg.market = Binance::new(None, None);

    let curr_prices = get_prices(&cfg.market)?;
    check_watched_symbols(&cfg, &curr_prices)?;
    Ok(())
}

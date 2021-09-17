use std::collections::HashMap;

use binance::{account::Account, market::Market, model::Prices};
use toml::Value;

use crate::app::{buy::buy_percent, notify::notify};

use super::{
    buy::get_balance,
    error::Error,
    sell::{sell, sell_all},
};

pub fn percent(old: f64, new: f64) -> f64 {
    ((new - old) / old) * 100.0
}

pub fn get_prices(market: &Market) -> Result<HashMap<String, f64>, Error> {
    match market.get_all_prices() {
        Ok(answer) => match answer {
            Prices::AllPrices(x) => {
                let mut res = HashMap::new();
                for u in x {
                    res.insert(u.symbol, u.price);
                }
                Ok(res)
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
            Err(Error {
                code: 1,
                message: "Failed to get all prices".to_string(),
            })
        }
    }
}

pub fn buy_new_tokens(
    old: &HashMap<String, f64>,
    new: &HashMap<String, f64>,
    owned: &mut HashMap<String, f64>,
    account: &Account,
    fiat: &str,
    cfg: &Value,
) -> Result<(), Error> {
    let mut res = HashMap::new();
    for (i, x) in new {
        if !old.contains_key(i) {
            res.insert(i.clone(), *x);
        }
    }
    if !res.is_empty() {
        let bal = get_balance(account)?;

        sell_all(new, &bal, account, cfg)?;
        for (v, _u) in res {
            println!("BUY {}", &v);
            buy_percent(30.0, account, &v, fiat, new[&v], new, cfg)?;
            // buy(None, account, &v, fiat, new[&v])?;
            owned.insert(v.to_string(), new[&v]);
        }
    }
    Ok(())
}

pub fn check_new(
    account: &Account,
    old: &HashMap<String, f64>,
    new: &HashMap<String, f64>,
    owned: &mut HashMap<String, f64>,
    threshhold: f64,
    fiat: &str,
    cfg: &Value,
) -> Result<(), Error> {
    let autobuy = cfg["autobuy_on_increase"].as_bool().unwrap();
    let simulate = cfg["simulate_autobuy"].as_bool().unwrap();
    if old.len() == new.len() {
        for (v, x) in old {
            let p = percent(*x, new[v]);
            if p >= threshhold && v.contains(fiat) && !owned.contains_key(v) {
                if autobuy {
                    buy_percent(
                        cfg["amount"].as_float().unwrap(),
                        account,
                        v,
                        fiat,
                        new[v],
                        new,
                        cfg,
                    )?;
                    owned.insert(v.to_string(), new[v]);
                } else if simulate {
                    owned.insert(v.to_string(), new[v]);
                }
                notify(
                    format!(
                        "Check out: {} @ ${} went up {} % over {} seconds",
                        &v,
                        new[v],
                        p,
                        cfg["time_interval"].as_integer().unwrap()
                            * cfg["num_intervals"].as_integer().unwrap()
                    ),
                    cfg,
                );
            }
        }
    } else {
        buy_new_tokens(old, new, owned, account, fiat, cfg)?
    }
    Ok(())
}

pub fn check_owned(
    owned: &mut HashMap<String, f64>,
    new: &HashMap<String, f64>,
    cfg: &Value,
    old: &HashMap<String, f64>,
    watched: &Vec<String>,
    account: &Account,
) -> Result<HashMap<String, f64>, Error> {
    let d = cfg["down"].as_float();
    let profit = cfg["take_profit"].as_float().unwrap();
    let simulate = cfg["simulate_autobuy"].as_bool().unwrap();
    let res: HashMap<String, f64> = HashMap::new();

    let down;
    let stoploss;
    if d == None {
        down = cfg["stop_loss"].as_float().unwrap();
        stoploss = true;
    } else {
        down = cfg["down"].as_float().unwrap();
        stoploss = false;
    }

    for (n, x) in owned {
        if !watched.contains(n) {
            let oldval;
            if stoploss {
                oldval = *x;
            } else {
                oldval = old[n];
            }
            let coin = n.replace(cfg["fiat"].as_str().unwrap(), "");
            if percent(oldval, new[n]) <= down {
                if new.contains_key(n) {
                    if simulate {
                        notify(format!("STOPLOSS - {} @ $ {}", n, new[n]), cfg);
                        continue;
                    }

                    sell(account, n, &coin, new[n], cfg)?;
                }
            }
            if percent(*x, new[n]) >= profit {
                if new.contains_key(n) {
                    if simulate {
                        notify(format!("TAKEPROFIT - {} @ $ {}", n, new[n]), cfg);
                        continue;
                    }
                    sell(account, n, &coin, new[n], cfg)?;
                }
            }
        }
    }
    Ok(res)
}

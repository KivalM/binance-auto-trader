use std::collections::HashMap;

use crate::{error::Error, general::ApiInfo};
use binance::{market::Market, model::Prices};

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
                message: e.to_string(),
            })
        }
    }
}

pub fn get_balance(cfg: &ApiInfo) -> Result<HashMap<String, f64>, Error> {
    let mut res = HashMap::new();
    match cfg.account.get_account() {
        Ok(answer) => {
            for i in answer.balances {
                let bal = i.free.parse::<f64>().unwrap();
                if bal > 0.0 {
                    res.insert(i.asset, bal);
                }
            }
            Ok(res)
        }
        Err(e) => {
            println!("{:?}", e);
            Err(Error {
                code: 2,
                message: e.to_string(),
            })
        }
    }
}

pub fn evaluate_balance(
    curr_prices: &HashMap<String, f64>,
    bal: &HashMap<String, f64>,
    cfg: &ApiInfo,
) -> f64 {
    let mut val = 0.0;
    let fiat = &cfg.config.base_pair;
    for (n, x) in bal {
        if n == fiat {
            val += x;
            continue;
        }
        let name = n.to_owned() + &fiat.to_owned();
        if curr_prices.contains_key(&name) {
            val += x * curr_prices[&name];
        }
    }
    val
}

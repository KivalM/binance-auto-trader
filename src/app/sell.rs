use std::collections::HashMap;

use binance::account::Account;
use toml::Value;

use crate::app::notify::notify;

use super::error::Error;

/// Will sell all of your current balance of a specified token
///
pub fn sell(
    account: &Account,
    symbol: &str,
    coin: &str,
    current_price: f64,
    cfg: &Value,
) -> Result<(), Error> {
    let amt = account.get_balance(coin);
    match amt {
        Ok(answer) => {
            let ans: f64 = answer.free.parse().unwrap();
            if ans > 0.004 {
                match account.market_sell(symbol, (ans * 1000.0).floor() / 1000.0) {
                    Ok(answer) => {
                        notify(format!("SELL {} @ {}", symbol, current_price), cfg);
                        println!("{:?}", answer);
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(Error {
                            code: 3,
                            message: format!("{:?}", &e),
                        });
                    }
                }
            } else {
                Ok(())
            }
        }

        Err(e) => {
            println!("{:?}", e);
            return Err(Error {
                code: 2,
                message: format!("Failed to get balance of {}", coin).to_string(),
            });
        }
    }
}

///
/// Will sell everything that you have a balance of
///
pub fn sell_all(
    prices: &HashMap<String, f64>,
    balances: &HashMap<String, f64>,
    account: &Account,
    cfg: &Value,
) -> Result<(), Error> {
    for x in balances.keys() {
        let n = x.to_owned() + cfg["fiat"].as_str().unwrap();
        if prices.contains_key(&n) {
            sell(account, &n, x, prices[&n], cfg)?;
        }
    }
    Ok(())
}

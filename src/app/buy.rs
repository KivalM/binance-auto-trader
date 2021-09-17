use std::collections::HashMap;

use binance::account::Account;
use toml::Value;

use crate::app::notify::notify;

use super::error::Error;
const DEPTH: f64 = 10000.0;
const MIN: f64 = 20.0;

/// will purchase a certain coin for a specific price
/// amount is the amount in `fiat`
/// if the total amount is not available,
/// it will try to purchase for the balance for the balance you have
/// if the amount is less than MIN, it will become MIN
/// and if its lower than account balance, it will become account balance

pub fn buy(
    amount: f64,
    account: &Account,
    symbol: &str,
    fiat: &str,
    current_price: f64,
    cfg: &Value,
) -> Result<(), Error> {
    let account_balance = account.get_balance(fiat);
    match account_balance {
        Ok(answer) => {
            let fiat_owned: f64 = answer.free.parse().unwrap();
            let final_amount = amount.max(MIN).min(fiat_owned) / current_price;
            let rounded = (final_amount * DEPTH).floor() / DEPTH;

            if (fiat_owned / current_price >= rounded) && rounded >= MIN {
                match account.market_buy(symbol, rounded) {
                    Ok(answer) => {
                        notify(format!("BUY: {} @ {}", symbol, current_price), cfg);
                        println!("{:?}", answer);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return Err(Error {
                            code: 2,
                            message: format!("{:?}", &e),
                        });
                    }
                }
            } else {
                return Err(Error {
                    code: 2,
                    message: format!("Insufficient Balance: {}", &rounded),
                });
            }
        }
        Err(_) => Err(Error {
            code: 1,
            message: "Failed to get account balance".to_string(),
        }),
    }
}

/// calculates the total value of all the coins in your account
/// and returns an f64 containing those values

pub fn evaluate_balance(
    curr_prices: &HashMap<String, f64>,
    bal: &HashMap<String, f64>,
    cfg: &Value,
) -> f64 {
    let mut val = 0.0;
    let fiat = cfg["fiat"].as_str().unwrap();
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

/// gets all of the balances from your account
/// and returns a hashmap containing all of those values

pub fn get_balance(account: &Account) -> Result<HashMap<String, f64>, Error> {
    let mut res = HashMap::new();
    match account.get_account() {
        Ok(answer) => {
            for i in answer.balances {
                let bal = i.free.parse::<f64>().unwrap();
                if bal > 0.0 {
                    res.insert(i.asset, bal);
                }
            }
            Ok(res)
        }
        Err(_) => Err(Error {
            code: 2,
            message: "Failed to get account information".to_string(),
        }),
    }
}

/// Will make a purchase of a coin using a percentage of your total account value

pub fn buy_percent(
    perc: f64,
    account: &Account,
    symbol: &str,
    fiat: &str,
    curr: f64,
    curr_prices: &HashMap<String, f64>,
    cfg: &Value,
) -> Result<(), Error> {
    let balances = get_balance(account)?;
    let acc_value = evaluate_balance(curr_prices, &balances, cfg);
    let tobuy = acc_value * perc / 100.0;
    buy(tobuy, account, symbol, fiat, curr, cfg)?;
    Ok(())
}

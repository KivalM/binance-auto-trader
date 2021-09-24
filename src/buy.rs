use std::collections::HashMap;

use crate::account::{evaluate_balance, get_balance};
use crate::config::Token;
use crate::general::get_step_size;
use crate::notify::notify;
use crate::{error::Error, general::ApiInfo};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
const MIN: f64 = 20.0;
const MAX_DIFF: f64 = 5.0;

pub fn conv_step(amount: f64, token: &Token, cfg: &ApiInfo) -> Result<Decimal, Error> {
    let dec_amt = Decimal::from_f64(amount).unwrap();
    let (min, step) = get_step_size(token, cfg)?;
    let n = ((dec_amt - min) / step).floor();
    let amt = min + (n * step);
    Ok(amt)
}

pub fn buy(amount: f64, token: &Token, cfg: &ApiInfo) -> Result<(), Error> {
    let t = token.symbol.replace(&token.base, "");
    let amount_owned: f64;

    let current_price: f64;
    match cfg.market.get_price(token.symbol.clone()) {
        Ok(price) => current_price = price.price,
        Err(e) => {
            println!("{:?}", e);
            return Err(Error {
                code: 1,
                message: e.to_string(),
            });
        }
    };
    match cfg.account.get_balance(t) {
        Ok(balance) => {
            amount_owned = balance.free.parse().unwrap();
            let token_amt = amount / current_price;
            println!("{}", amount_owned);
            if (amount_owned - token_amt).abs() <= MAX_DIFF || amount_owned > token_amt {
                println!("{}, {}", amount_owned, token_amt);
                return Ok(());
            }
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(Error {
                code: 1,
                message: e.to_string(),
            });
        }
    }

    let account_balance = cfg.account.get_balance(token.base.clone());
    match account_balance {
        Ok(answer) => {
            let free_balance: f64 = answer.free.parse().unwrap();

            let mut final_amount = (amount - amount_owned).abs();
            final_amount.max(MIN).min(free_balance);
            final_amount /= current_price;
            let rounded_dec = conv_step(final_amount, token, cfg)?;
            let rounded: f64 = rounded_dec.to_string().parse().unwrap();
            if (free_balance / current_price >= rounded) && rounded >= MIN {
                match cfg.account.market_buy(token.symbol.clone(), rounded) {
                    Ok(answer) => {
                        println!("{:?}", answer);
                        notify(format!("BUY: {} @ {}", token.symbol, current_price), cfg);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        return Err(Error {
                            code: 1,
                            message: e.to_string(),
                        });
                    }
                }
            }

            Ok(())
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(Error {
                code: 1,
                message: e.to_string(),
            });
        }
    }
}
pub fn buy_percent(
    token: &Token,
    curr_prices: &HashMap<String, f64>,
    cfg: &ApiInfo,
) -> Result<(), Error> {
    let balances = get_balance(cfg)?;
    let acc_value = evaluate_balance(curr_prices, &balances, cfg);
    let tobuy = acc_value * token.ratio / 100.0;
    buy(tobuy, token, cfg)?;
    Ok(())
}

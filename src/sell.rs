use crate::{buy::conv_step, config::Token, error::Error, general::ApiInfo, notify::notify};
use rust_decimal::prelude::*;
use std::collections::HashMap;
pub fn sell(token: &Token, cfg: &ApiInfo, curr_prices: &HashMap<String, f64>) -> Result<(), Error> {
    let amt = cfg.account.get_balance(token.token.clone());
    match amt {
        Ok(answer) => {
            let ans: f64 = answer.free.parse().unwrap();
            let min = 20.0 / curr_prices[&token.symbol];

            if ans > min {
                let rounded_dec: Decimal = conv_step(ans, token, cfg)?;
                let rounded: f64 = rounded_dec.to_string().parse().unwrap();
                println!("{}", rounded);
                match cfg.account.market_sell(token.symbol.clone(), rounded) {
                    Ok(_) => {
                        notify(
                            format!("SELL {} @ {}", token.symbol, curr_prices[&token.symbol]),
                            cfg,
                        );
                        println!("{:?}", answer);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        return Err(Error {
                            code: 3,
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
                code: 3,
                message: e.to_string(),
            });
        }
    }
}

use std::collections::HashMap;

use crate::{buy::conv_step, config::Token, error::Error, general::ApiInfo, notify::notify};

pub fn sell(token: &Token, cfg: &ApiInfo, curr_prices: &HashMap<String, f64>) -> Result<(), Error> {
    let amt = cfg.account.get_balance(token.token.clone());
    match amt {
        Ok(answer) => {
            let ans: f64 = answer.free.parse().unwrap();
            let min = 20.0 / curr_prices[&token.symbol];

            if ans > min {
                let rounded = conv_step(ans, token, cfg)?;
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
                        return Err(Error {
                            code: 3,
                            message: e.to_string(),
                        })
                    }
                }
            }

            Ok(())
        }
        Err(e) => {
            return Err(Error {
                code: 3,
                message: e.to_string(),
            })
        }
    }
}

use binance::{account::Account, general::General, market::Market};

use crate::{
    config::{Config, Token},
    error::Error,
};
#[derive(Clone)]
pub struct ApiInfo {
    pub account: Account,
    pub market: Market,
    pub general: General,
    pub config: Config,
}

pub fn get_step_size(token: &Token, cfg: &ApiInfo) -> Result<(f64, f64), Error> {
    match cfg.general.get_symbol_info(token.symbol.clone()) {
        Ok(answer) => {
            for i in answer.filters {
                match i {
                    binance::model::Filters::LotSize {
                        min_qty,
                        max_qty: _,
                        step_size,
                    } => return Ok((min_qty.parse().unwrap(), step_size.parse().unwrap())),

                    _ => {
                        continue;
                    }
                }
            }
        }
        Err(e) => {
            return Err(Error {
                code: 1,
                message: e.to_string(),
            })
        }
    }
    Err(Error {
        code: 1,
        message: "this shouldnt happen".to_string(),
    })
}

pub fn percent(old: f64, new: f64) -> f64 {
    ((new - old) / old) * 100.0
}

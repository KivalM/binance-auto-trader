use std::str::FromStr;

use binance::{account::Account, general::General, market::Market};
use rust_decimal::Decimal;

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

pub fn get_step_size(token: &Token, cfg: &ApiInfo) -> Result<(Decimal, Decimal), Error> {
    match cfg.general.get_symbol_info(token.symbol.clone()) {
        Ok(answer) => {
            for i in answer.filters {
                match i {
                    binance::model::Filters::LotSize {
                        min_qty,
                        max_qty: _,
                        step_size,
                    } => {
                        let min = Decimal::from_str(&min_qty).unwrap();
                        let step = Decimal::from_str(&step_size).unwrap();
                        return Ok((min, step));
                    }

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

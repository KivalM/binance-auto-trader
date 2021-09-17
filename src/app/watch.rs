use std::{collections::HashMap, convert::TryInto};

use binance::{account::Account, market::Market};
use toml::Value;

use super::{account::percent, buy::buy_percent, error::Error, sell::sell};

pub fn get_kline_stats(
    market: &Market,
    time_down: usize,
    time_up: usize,
    symbol: &str,
) -> Result<(f64, f64, f64), Error> {
    // Gets the current price of the determinant, the price TIME_UP ago, and TIME_DOWN ago

    let max = time_up.max(time_down);
    let lim = (max + 1) as u16;

    match market.get_klines(symbol, "5m", lim, None, None) {
        Ok(klines) => match klines {
            binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                let currprice = klines[max].to_owned().open;
                let downprice = klines[max - time_down].to_owned().open;
                let upprice = klines[max - time_up].to_owned().open;
                Ok((currprice, downprice, upprice))
            }
        },
        Err(e) => {
            println!("FAILED {:?}", e);
            Err(Error {
                code: 3,
                message: "Failed to get Kline Stats".to_string(),
            })
        }
    }
}

pub fn check_watched_symbols(
    market: &Market,
    account: &Account,
    cfg: &Value,
    curr_prices: &HashMap<String, f64>,
) -> Result<(), Error> {
    let toks = cfg["tokens"].as_table().unwrap();
    for tok in toks.keys() {
        let u = &toks[tok];
        // println!("{:?}", i.len());
        let (curr, down, up) = get_kline_stats(
            market,
            u["time_down"].as_integer().unwrap().try_into().unwrap(),
            u["time_up"].as_integer().unwrap().try_into().unwrap(),
            u["symbol"].as_str().unwrap(),
        )?;
        let to_buy = percent(up, curr) >= u["perc_up"].as_float().unwrap();
        let to_sell = percent(down, curr) <= u["perc_down"].as_float().unwrap();

        if to_buy && to_sell {
            return Ok(());
        } else if to_buy {
            buy_percent(
                u["ratio"].as_float().unwrap(),
                account,
                u["symbol"].as_str().unwrap(),
                u["fiat"].as_str().unwrap(),
                curr,
                curr_prices,
                cfg,
            )?
        } else if to_sell {
            sell(
                account,
                u["symbol"].as_str().unwrap(),
                u["token"].as_str().unwrap(),
                curr,
                cfg,
            )?
        }
    }
    Ok(())
}

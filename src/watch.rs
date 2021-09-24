use std::collections::HashMap;

use crate::{
    buy::buy_percent,
    error::Error,
    general::{percent, ApiInfo},
    market::get_kline_stats,
    sell::sell,
};

pub fn check_watched_symbols(
    cfg: &ApiInfo,
    curr_prices: &HashMap<String, f64>,
) -> Result<(), Error> {
    let tokens = &cfg.config.tokens;

    for i in tokens {
        let (curr, down, up) = get_kline_stats(i, cfg)?;
        println!("C {} d {} u {}", curr, down, up);
        let to_buy = percent(up, curr) >= i.perc_up;
        let to_sell = percent(down, curr) <= i.perc_down;
        println!("b {} s {}", to_buy, to_sell);
        if to_buy && to_sell {
            continue;
        } else if to_buy {
            buy_percent(i, curr_prices, cfg)?;
        } else if to_sell {
            sell(i, cfg, curr_prices)?;
        }
    }
    Ok(())
}

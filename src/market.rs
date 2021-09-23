use crate::{config::Token, error::Error, general::ApiInfo};

pub fn get_kline_stats(token: &Token, cfg: &ApiInfo) -> Result<(f64, f64, f64), Error> {
    let max = token.time_up.max(token.time_down) as usize;
    let lim = (max + 1) as u16;

    match cfg
        .market
        .get_klines(token.symbol.clone(), "5m", lim, None, None)
    {
        Ok(klines) => match klines {
            binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                let currprice = klines[max].to_owned().open;
                let downprice = klines[max - token.time_down as usize].to_owned().open;
                let upprice = klines[max - token.time_up as usize].to_owned().open;
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

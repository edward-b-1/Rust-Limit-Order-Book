
use crate::limit_order_book_lib::MultiTickerLimitOrderBook;
use crate::limit_order_book_lib::OrderSide;

use ordered_float::NotNan;


pub fn profit_function_sell<'s, 'l>(
    limit_order_book: &'l mut MultiTickerLimitOrderBook<'s>,
    ticker: &'s str,
    mut target_volume: NotNan<f64>,
) -> f64 {
    let total_volume = limit_order_book.total_volume(ticker, &OrderSide::BUY);

    let mut total_cost = 0.0;

    for (price, volume) in total_volume.iter().rev() {
        //println!("in profit_function_sell: inspecting price level {price}, which has volume {volume}, these should be descending");
        let volume_to_take = std::cmp::min(*volume, target_volume).into_inner(); // cannot be NaN
        let cost_of_taking = volume_to_take * price.into_inner();
        total_cost += cost_of_taking;
        target_volume -= volume_to_take;
        if *target_volume <= 0.0 {
            break;
        }
    }

    total_cost
}

pub fn cost_function_buy<'s, 'l>(
    limit_order_book: &'l mut MultiTickerLimitOrderBook<'s>,
    ticker: &'s str,
    mut target_volume: NotNan<f64>,
) -> f64 {
    let total_volume = limit_order_book.total_volume(ticker, &OrderSide::SELL);

    let mut total_cost = 0.0;

    for (price, volume) in total_volume.iter() {
        //println!("in cost_function_buy: inspecting price level {price}, which has volume {volume}, these should be ascending");
        let volume_to_take = std::cmp::min(*volume, target_volume).into_inner(); // cannot be NaN
        let cost_of_taking = volume_to_take * price.into_inner();
        total_cost += cost_of_taking;
        target_volume -= volume_to_take;
        if *target_volume <= 0.0 {
            break;
        }
    }

    total_cost
}
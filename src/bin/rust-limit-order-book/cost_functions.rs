
use crate::limit_order_book_lib::MultiTickerLimitOrderBook;
use crate::limit_order_book_lib::OrderSide;

use std::collections::BTreeMap;

use ordered_float::NotNan;


pub fn profit_function_sell<'s, 'l>(
    limit_order_book: &'l mut MultiTickerLimitOrderBook<'s>,
    ticker: &'s str,
    mut target_volume: NotNan<f64>,
) -> f64 {
    let total_volume = limit_order_book.total_volume_by_price_level(ticker, &OrderSide::BUY);

    let mut total_profit = 0.0;

    for (price, volume) in total_volume.iter().rev() {
        //println!("in profit_function_sell: inspecting price level {price}, which has volume {volume}, these should be descending");
        let volume_to_sell = std::cmp::min(*volume, target_volume).into_inner(); // cannot be NaN
        let profit_from_selling = volume_to_sell * price.into_inner();
        total_profit += profit_from_selling;
        target_volume -= volume_to_sell;
        if *target_volume <= 0.0 {
            break;
        }
    }

    total_profit
}

pub fn profit_function_sell_with_source_exchange<'s, 'l>(
    limit_order_book: &'l mut MultiTickerLimitOrderBook<'s>,
    ticker: &'s str,
    mut target_volume: NotNan<f64>,
) -> BTreeMap<&'l str, f64> {
    let total_volume_by_source_exchange = limit_order_book.total_volume_by_price_level_and_source_exchange(ticker, &OrderSide::SELL);

    let mut total_profit_by_source_exchange = BTreeMap::new();

    for (price, volume_by_source_exchange) in total_volume_by_source_exchange.iter() {
        for (source_exchange, volume) in volume_by_source_exchange.iter() {
            let volume_to_sell = std::cmp::min(*volume, target_volume).into_inner();
            let profit_from_selling = volume_to_sell * price.into_inner();
            let profit_from_selling_by_source_exchange = 
                total_profit_by_source_exchange.entry(*source_exchange).or_insert(f64::default());
            *profit_from_selling_by_source_exchange += profit_from_selling;
            target_volume -= volume_to_sell;
            if *target_volume <= 0.0 {
                break;
            }
        }
    }

    total_profit_by_source_exchange
}

pub fn cost_function_buy<'s, 'l>(
    limit_order_book: &'l mut MultiTickerLimitOrderBook<'s>,
    ticker: &'s str,
    mut target_volume: NotNan<f64>,
) -> f64 {
    let total_volume = limit_order_book.total_volume_by_price_level(ticker, &OrderSide::SELL);

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

pub fn cost_function_buy_with_source_exchange<'s, 'l>(
    limit_order_book: &'l mut MultiTickerLimitOrderBook<'s>,
    ticker: &'s str,
    mut target_volume: NotNan<f64>,
) -> BTreeMap<&'l str, f64> {
    let total_volume_by_source_exchange = limit_order_book.total_volume_by_price_level_and_source_exchange(ticker, &OrderSide::SELL);

    let mut total_cost_by_source_exchange = BTreeMap::new();

    for (price, volume_by_source_exchange) in total_volume_by_source_exchange.iter() {
        for (source_exchange, volume) in volume_by_source_exchange.iter() {
            let volume_to_take = std::cmp::min(*volume, target_volume).into_inner();
            let cost_of_taking = volume_to_take * price.into_inner();
            let cost_of_taking_by_source_exchange = 
                total_cost_by_source_exchange.entry(*source_exchange).or_insert(f64::default());
            *cost_of_taking_by_source_exchange += cost_of_taking;
            target_volume -= volume_to_take;
            if *target_volume <= 0.0 {
                break;
            }
        }
    }

    total_cost_by_source_exchange
}
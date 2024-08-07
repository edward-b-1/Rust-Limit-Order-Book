
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use ordered_float::NotNan;

use super::SingleSideLimitOrderBook;
use super::OrderSide;
use super::Order;


#[derive(Debug)]
pub struct DoubleSideLimitOrderBook<'s> {
    ticker: &'s str,
    buy_side_limit_order_book: SingleSideLimitOrderBook<'s>,
    sell_side_limit_order_book: SingleSideLimitOrderBook<'s>,
}

impl<'s> DoubleSideLimitOrderBook<'s> {
    pub fn new(ticker: &'s str) -> DoubleSideLimitOrderBook {
        DoubleSideLimitOrderBook {
            ticker,
            buy_side_limit_order_book: SingleSideLimitOrderBook::new(OrderSide::BUY),
            sell_side_limit_order_book: SingleSideLimitOrderBook::new(OrderSide::SELL),
        }
    }

    pub fn add_order(&mut self, order: Order<'s>) {
        assert!(order.ticker == self.ticker);
        match order.order_side {
            OrderSide::BUY => {
                self.buy_side_limit_order_book.add_order(order)
            },
            OrderSide::SELL => {
                self.sell_side_limit_order_book.add_order(order)
            }
        }
    }

    pub fn total_volume_by_price_level(&self, order_side: &OrderSide)
        -> BTreeMap<NotNan<f64>, NotNan<f64>>
    {
        match *order_side {
            OrderSide::BUY => {
                self.buy_side_limit_order_book.total_volume_by_price_level()
            },
            OrderSide::SELL => {
                self.sell_side_limit_order_book.total_volume_by_price_level()
            },
        }
    }

    pub fn total_volume_by_source_exchange(&self, order_side: &OrderSide)
        -> BTreeMap<&str, NotNan<f64>>
    {
        match *order_side {
            OrderSide::BUY => {
                self.buy_side_limit_order_book.total_volume_by_source_exchange()
            },
            OrderSide::SELL => {
                self.sell_side_limit_order_book.total_volume_by_source_exchange()
            },
        }
    }

    pub fn total_volume_by_price_level_and_source_exchange(&self, order_side: &OrderSide)
        -> BTreeMap<NotNan<f64>, BTreeMap<&str, NotNan<f64>>>
    {
        match *order_side {
            OrderSide::BUY => {
                self.buy_side_limit_order_book.total_volume_by_price_level_and_source_exchange()
            },
            OrderSide::SELL => {
                self.sell_side_limit_order_book.total_volume_by_price_level_and_source_exchange()
            },
        }
    }

    pub fn clear(&mut self) {
        self.buy_side_limit_order_book.clear();
        self.sell_side_limit_order_book.clear();
    }

    pub fn highest_bid_price_by_exchange(&self) -> BTreeMap<&str, NotNan<f64>> {
        self.buy_side_limit_order_book.highest_price_by_exchange()
    }

    pub fn lowest_ask_price_by_exchange(&self) -> BTreeMap<&str, NotNan<f64>> {
        self.sell_side_limit_order_book.lowest_price_by_exchange()
    }

    pub fn spread(&mut self) -> Option<NotNan<f64>> {
        let buy_price = self.buy_side_limit_order_book.highest_price();
        let sell_price = self.sell_side_limit_order_book.lowest_price();
        match (buy_price, sell_price) {
            (Some(buy_price), Some(sell_price)) => {
                Some(sell_price - buy_price)
            },
            _ => {
                None
            },
        }
    }

    pub fn spread_by_exchange(&mut self) -> BTreeMap<&str, Option<NotNan<f64>>> {
        let buy_price_by_exchange = self.buy_side_limit_order_book.highest_price_by_exchange();
        let sell_price_by_exchange = self.sell_side_limit_order_book.lowest_price_by_exchange();

        let buy_price_exchanges = buy_price_by_exchange.keys().cloned().collect::<BTreeSet<&str>>();
        let sell_price_exchanges = sell_price_by_exchange.keys().cloned().collect::<BTreeSet<&str>>();
        let exchanges = buy_price_exchanges.union(&sell_price_exchanges);

        let mut spread_by_exchange = BTreeMap::new();

        for exchange in exchanges {
            let existing_entry = spread_by_exchange.entry(*exchange).or_insert(None);

            let optional_buy_price = buy_price_by_exchange.get(exchange);
            let optional_sell_price = sell_price_by_exchange.get(exchange);

            match (optional_buy_price, optional_sell_price) {
                (Some(buy_price), Some(sell_price)) => {
                    let spread = sell_price - buy_price;
                    *existing_entry = Some(spread);
                },
                _ => {
                    // pass
                },
            }
        }

        spread_by_exchange
    }
}
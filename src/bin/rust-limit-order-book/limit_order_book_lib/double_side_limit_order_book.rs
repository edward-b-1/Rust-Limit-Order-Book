
use std::collections::BTreeMap;

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
}
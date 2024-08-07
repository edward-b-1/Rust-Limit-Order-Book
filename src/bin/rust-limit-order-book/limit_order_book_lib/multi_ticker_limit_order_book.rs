
use std::collections::BTreeMap;

use ordered_float::NotNan;

use super::DoubleSideLimitOrderBook;
use super::OrderSide;
use super::Order;


#[derive(Debug)]
pub struct MultiTickerLimitOrderBook<'s> {
    double_limit_order_books: BTreeMap<&'s str, DoubleSideLimitOrderBook<'s>>,
}

impl<'s> MultiTickerLimitOrderBook<'s> {
    pub fn new() -> MultiTickerLimitOrderBook<'s> {
        MultiTickerLimitOrderBook {
            double_limit_order_books: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order<'s>) {
        let ticker = &order.ticker;
        let double_side_limit_order_book =
            self.double_limit_order_books
                .entry(ticker)
                .or_insert(
                    DoubleSideLimitOrderBook::new(ticker),
                );
        double_side_limit_order_book.add_order(order);
    }

    pub fn total_volume_by_price_level(&mut self, ticker: &'s str, order_side: &OrderSide)
        -> BTreeMap<NotNan<f64>, NotNan<f64>>
    {
        let double_side_limit_order_book = 
            self.double_limit_order_books
                .entry(ticker)
                .or_insert(
                    DoubleSideLimitOrderBook::new(ticker)
                );
        double_side_limit_order_book.total_volume_by_price_level(order_side)
    }

    pub fn total_volume_by_price_level_and_source_exchange(&mut self, ticker: &'s str, order_side: &OrderSide)
        -> BTreeMap<NotNan<f64>, BTreeMap<&str, NotNan<f64>>>
    {
        let double_side_limit_order_book = 
            self.double_limit_order_books
                .entry(ticker)
                .or_insert(
                    DoubleSideLimitOrderBook::new(ticker)
                );
        double_side_limit_order_book.total_volume_by_price_level_and_source_exchange(order_side)
    }

    pub fn total_volume_by_source_exchange(&mut self, ticker: &'s str, order_side: &OrderSide)
        -> BTreeMap<&str, NotNan<f64>>
    {
        let double_side_limit_order_book =
            self.double_limit_order_books.
                entry(ticker)
                .or_insert(
                    DoubleSideLimitOrderBook::new(ticker)
                );
        double_side_limit_order_book.total_volume_by_source_exchange(order_side)
    }

    pub fn clear(&mut self) {
        self.double_limit_order_books.clear();
    }

    pub fn debug_print(&self) {
        let lob = &self.double_limit_order_books;
        println!("{lob:?}");
    }

    pub fn highest_bid_price_by_exchange(&mut self, ticker: &'s str)
        -> BTreeMap<&str, NotNan<f64>>
    {
        self.double_limit_order_books
            .entry(ticker)
            .or_insert(DoubleSideLimitOrderBook::new(ticker))
            .highest_bid_price_by_exchange()
    }

    pub fn lowest_ask_price_by_exchange(&mut self, ticker: &'s str)
        -> BTreeMap<&str, NotNan<f64>>
    {
        self.double_limit_order_books
            .entry(ticker)
            .or_insert(DoubleSideLimitOrderBook::new(ticker))
            .lowest_ask_price_by_exchange()
    }

    pub fn spread(&mut self, ticker: &'s str) -> Option<NotNan<f64>> {
        let double_side_limit_order_book =
            self.double_limit_order_books
                .entry(ticker)
                .or_insert(
                    DoubleSideLimitOrderBook::new(ticker)
                );
        double_side_limit_order_book.spread()
    }

    pub fn spread_by_exchange(&mut self, ticker: &'s str) -> BTreeMap<&str, Option<NotNan<f64>>> {
        let double_side_limit_order_book =
            self.double_limit_order_books
                .entry(ticker)
                .or_insert(
                    DoubleSideLimitOrderBook::new(ticker)
                );
        double_side_limit_order_book.spread_by_exchange()
    }
}
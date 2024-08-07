
use std::collections::VecDeque;
use std::collections::BTreeMap;

use ordered_float::NotNan;

use super::Order;


#[derive(Debug)]
pub struct PriceLevel<'s> {
    /// Example use of PriceLevel
    ///
    /// let source_exchange_coinbase = "COINBASE";
    /// let source_exchange_gemini = "GEMINI";
    /// let ticker = "example ticker";
    /// 
    /// let order1 = limit_order_book_lib::Order::new(
    ///     ticker,
    ///     limit_order_book_lib::OrderSide::BUY,
    ///     100.0,
    ///     20.0,
    ///     source_exchange_coinbase,
    /// ).unwrap();
    ///
    /// let order2 = limit_order_book_lib::Order::new(
    ///     ticker,
    ///     limit_order_book_lib::OrderSide::BUY,
    ///     100.0,
    ///     10.0,
    ///     source_exchange_coinbase,
    /// ).unwrap();
    ///
    /// let order3 = limit_order_book_lib::Order::new(
    ///     ticker,
    ///     limit_order_book_lib::OrderSide::BUY,
    ///     100.0,
    ///     5.0,
    ///     source_exchange_gemini,
    /// ).unwrap();
    ///
    /// let orders = VecDeque::from(
    ///     vec![order1, order2, order3],
    /// );
    ///
    /// use ordered_float::NotNan;
    /// let mut price_level = limit_order_book_lib::PriceLevel::new(NotNan::new(100.0).unwrap());
    /// for order in orders {
    ///     price_level.add_order(order);
    /// }
    ///
    /// println!("{price_level:?}");

    price: NotNan<f64>,
    orders: VecDeque<Order<'s>>,
}

impl <'s> PriceLevel<'s> {
    pub fn new(price: NotNan<f64>) -> PriceLevel<'s> {
        PriceLevel {
            price,
            orders: VecDeque::new(),
        }
    }

    pub fn add_order(&mut self, order: Order<'s>) {
        assert!(order.price == self.price);
        self.orders.push_back(order);
    }

    pub fn total_volume_by_source_exchange(&self)
        -> BTreeMap<&str, NotNan<f64>>
    {
        let mut total_volume_by_source_exchange = BTreeMap::new();
        for order in &self.orders {
            let source_exchange = order.source_exchange;
            let total_volume = total_volume_by_source_exchange.entry(source_exchange).or_insert(NotNan::default());
            *total_volume += order.volume;
        }
        total_volume_by_source_exchange
    }

    pub fn total_volume_with_price_level(&self) -> (NotNan<f64>, NotNan<f64>) {
        let mut total_volume = NotNan::default();
        for order in &self.orders {
            total_volume += order.volume;
        }
        (self.price, total_volume)
    }

    pub fn total_volume_by_source_exchange_with_price_level(&self)
        -> (NotNan<f64>, BTreeMap<&str, NotNan<f64>>)
    {
        (self.price, self.total_volume_by_source_exchange())
    }

    pub fn clear(&mut self) {
        self.orders.clear()
    }
}
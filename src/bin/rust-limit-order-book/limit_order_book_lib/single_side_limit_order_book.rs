
use std::collections::BTreeMap;

use ordered_float::NotNan;

use super::PriceLevel;
use super::OrderSide;
use super::Order;


#[derive(Debug)]
pub struct SingleSideLimitOrderBook<'s> {
    order_side: OrderSide,
    price_levels: BTreeMap<NotNan<f64>, PriceLevel<'s>>,
}

impl<'s> SingleSideLimitOrderBook<'s> {
    pub fn new(order_side: OrderSide) -> SingleSideLimitOrderBook<'s> {
        SingleSideLimitOrderBook {
            order_side,
            price_levels: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order<'s>) {
        assert!(order.order_side == self.order_side); // TODO: maybe some proper error type here
        let price = order.price;
        let price_level = self.price_levels.entry(price).or_insert(PriceLevel::new(price));
        price_level.add_order(order);
    }

    pub fn total_volume_by_price_level(&self) -> BTreeMap<NotNan<f64>, NotNan<f64>> {
        let total_volume_by_price_level = self.price_levels.values().map(
            |price_level| {
                price_level.total_volume_with_price_level()
            }
        ).collect();
        total_volume_by_price_level
    }

    pub fn total_volume_by_source_exchange(&self)
        -> BTreeMap<&str, NotNan<f64>>
    {
        let mut total_volume_by_source_exchange = BTreeMap::new();

        for price_level in self.price_levels.values() {
            let price_level_total_volume_by_source_exchange = price_level.total_volume_by_source_exchange();
            for (source_exchange, total_volume) in price_level_total_volume_by_source_exchange {
                let existing_total_volume = total_volume_by_source_exchange.entry(source_exchange).or_insert(NotNan::default());
                *existing_total_volume += total_volume;
            }
        }
        
        total_volume_by_source_exchange
    }

    pub fn total_volume_by_price_level_and_source_exchange(&self)
        -> BTreeMap<NotNan<f64>, BTreeMap<&str, NotNan<f64>>>
    {
        let total_volume_by_price_level_and_source_exchange = self.price_levels.values().map(
            |price_level| {
                price_level.total_volume_by_source_exchange_with_price_level()
            }
        ).collect();
        total_volume_by_price_level_and_source_exchange
    }

    pub fn clear(&mut self) {
        self.price_levels.clear()
    }

}
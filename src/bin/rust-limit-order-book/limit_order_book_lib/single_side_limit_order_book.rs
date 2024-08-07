
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

    pub fn highest_price(&self) -> Option<NotNan<f64>> {
        self.price_levels
            .iter()
            .filter(
                |price_level| {
                    price_level.1.total_volume_with_price_level().1.into_inner() > 0.0
                }
            )
            .last()
            .map(
                |price_level| {
                    price_level.1.total_volume_with_price_level().0
                }
            )
    }

    pub fn lowest_price(&self) -> Option<NotNan<f64>> {
        self.price_levels
            .iter()
            .filter(
                |price_level| {
                    price_level.1.total_volume_with_price_level().1.into_inner() > 0.0
                }
            )
            .next()
            .map(
                |price_level| {
                    price_level.1.total_volume_with_price_level().0
                }
            )
    }

    pub fn highest_price_by_exchange(&self) -> BTreeMap<&str, NotNan<f64>> {
        let mut highest_price_by_exchange = BTreeMap::new();

        // NOTE: This function can report that some exchanges have zero volume
        // because the highest price differs across exchanges
        // self.price_levels
        //     .iter()
        //     .last()
        //     .map(
        //         |price_level| {
        //             let total_volume_by_source_exchange = price_level.1.total_volume_by_source_exchange();
        //             for (source_exchange, volume) in total_volume_by_source_exchange {
        //                 let entry = highest_price_by_exchange.entry(source_exchange).or_insert(NotNan::default());
        //                 *entry += volume;
        //             }
        //         }
        //     );

        for price_level in &self.price_levels {
            let price_level = price_level.1;
            let total_volume_by_source_exchange_with_price_level =
                price_level.total_volume_by_source_exchange_with_price_level();
            let price = total_volume_by_source_exchange_with_price_level.0;
            let total_volume_by_source_exchange = total_volume_by_source_exchange_with_price_level.1;

            for total_volume_and_source_exchange in total_volume_by_source_exchange {
                let source_exchange = total_volume_and_source_exchange.0;
                let existing_entry =
                    highest_price_by_exchange.entry(source_exchange).or_insert(price);
                if price > *existing_entry {
                    *existing_entry = price;
                }
            }
        }

        highest_price_by_exchange
    }

    pub fn lowest_price_by_exchange(&self) -> BTreeMap<&str, NotNan<f64>> {
        let mut lowest_price_by_exchange = BTreeMap::new();

        for price_level in &self.price_levels {
            let price_level = price_level.1;
            let total_volume_by_source_exchange_with_price_level = 
                price_level.total_volume_by_source_exchange_with_price_level();
            let price = total_volume_by_source_exchange_with_price_level.0;
            let total_volume_by_source_exchange = total_volume_by_source_exchange_with_price_level.1;

            for total_volume_and_source_exchange in total_volume_by_source_exchange {
                let source_exchange = total_volume_and_source_exchange.0;
                let existing_entry = 
                    lowest_price_by_exchange.entry(source_exchange).or_insert(price);
                if price < *existing_entry {
                    *existing_entry = price;
                }
            }
        }

        // NOTE: This function can report that some exchanges have zero volume
        // because the lowest price differs across exchanges
        // self.price_levels
        //     .iter()
        //     .next()
        //     .map(
        //         |price_level| {
        //             let total_volume_by_source_exchange = price_level.1.total_volume_by_source_exchange();
        //             for (source_exchange, volume) in total_volume_by_source_exchange {
        //                 let entry = lowest_price_by_exchange.entry(source_exchange).or_insert(NotNan::default());
        //                 *entry += volume;
        //             }
        //         }
        //     );

        lowest_price_by_exchange
    }
}
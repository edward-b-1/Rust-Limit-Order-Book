
mod price_level;
mod single_side_limit_order_book;
mod double_side_limit_order_book;
mod multi_ticker_limit_order_book;

pub use price_level::PriceLevel;
pub use single_side_limit_order_book::SingleSideLimitOrderBook;
pub use double_side_limit_order_book::DoubleSideLimitOrderBook;
pub use multi_ticker_limit_order_book::MultiTickerLimitOrderBook;

use std::fmt;
use std::str::FromStr;

use ordered_float::NotNan;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OrderSide {
    BUY,
    SELL,
}

#[derive(Debug, Clone)]
pub struct OrderSideParseError {
    input: String,
}

impl std::error::Error for OrderSideParseError {

}

impl fmt::Display for OrderSideParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let input = self.input.as_str();
        write!(f, "{input} is not a valid OrderSide")
    }
}

impl FromStr for OrderSide {
    type Err = OrderSideParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => {
                Ok(OrderSide::BUY)
            },
            "SELL" => {
                Ok(OrderSide::SELL)
            },
            _ => {
                Err(
                    OrderSideParseError {
                        input: String::from(s),
                    }
                )
            },
        }
    }
}

#[derive(Debug)]
pub struct Order<'s> {
    ticker: &'s str,
    order_side: OrderSide,
    price: NotNan<f64>,
    volume: NotNan<f64>,
    source_exchange: &'s str, // TODO: know the price to buy/sell X BTC but don't know who has it
}
    
impl<'s> Order<'s> {
    pub fn new(
        ticker: &'s str,
        order_side: OrderSide,
        price: f64,
        volume: f64,
        source_exchange: &'s str,
    ) -> Result<Order<'s>, ordered_float::FloatIsNan> {
        Ok(
            Order {
                ticker,
                order_side,
                price: NotNan::new(price)?,
                volume: NotNan::new(volume)?,
                source_exchange,
            }
        )
    }
}


#[cfg(test)]
mod tests;

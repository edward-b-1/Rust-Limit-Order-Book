
use std::str::FromStr;
use std::fmt;
use ordered_float::NotNan;
use std::collections::VecDeque;
use std::collections::BTreeMap;

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
    source_exchange: &'s str,
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

#[derive(Debug)]
pub struct PriceLevel<'s> {
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

    pub fn total_volume(&self) -> (NotNan<f64>, NotNan<f64>) {
        let mut total_volume = NotNan::default();
        for order in &self.orders {
            total_volume += order.volume;
        }
        (self.price, total_volume)
    }

    pub fn clear(&mut self) {
        self.orders.clear()
    }
}

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

    pub fn total_volume(&self) -> BTreeMap<NotNan<f64>, NotNan<f64>> {
        let total_volume = self.price_levels.values().map(
            |price_level| {
                price_level.total_volume()
            }
        ).collect();
        total_volume
    }

    pub fn clear(&mut self) {
        self.price_levels.clear()
    }
}

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

    pub fn total_volume_buy(&self) -> BTreeMap<NotNan<f64>, NotNan<f64>> {
        self.buy_side_limit_order_book.total_volume()
    }

    pub fn total_volume_sell(&self) -> BTreeMap<NotNan<f64>, NotNan<f64>> {
        self.sell_side_limit_order_book.total_volume()
    }

    pub fn total_volume(&self, order_side: &OrderSide) -> BTreeMap<NotNan<f64>, NotNan<f64>> {
        match *order_side {
            OrderSide::BUY => {
                self.total_volume_buy()
            },
            OrderSide::SELL => {
                self.total_volume_sell()
            },
        }
    }

    pub fn clear(&mut self) {
        self.buy_side_limit_order_book.clear();
        self.sell_side_limit_order_book.clear();
    }
}

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

    pub fn total_volume(&mut self, ticker: &'s str, order_side: &OrderSide) -> BTreeMap<NotNan<f64>, NotNan<f64>> {
        let double_side_limit_order_book = 
            self.double_limit_order_books
                .entry(ticker)
                .or_insert(
                    DoubleSideLimitOrderBook::new(ticker)
                );
        double_side_limit_order_book.total_volume(order_side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn price_level_test() {
        
        let price = 100.0;
        let mut price_level = PriceLevel::new(NotNan::new(price).unwrap());

        let ticker = "EXAMPLE";
        let order_side = OrderSide::BUY;
        let source_exchange_1 = "SRCEX1";
        let source_exchange_2 = "SRCEX2";

        let order_1 = Order::new(
            ticker,
            order_side,
            price,
            20.0,
            source_exchange_1,
        ).unwrap();

        let order_2 = Order::new(
            ticker,
            order_side,
            price,
            25.0,
            source_exchange_1,
        ).unwrap();

        let order_3 = Order::new(
            ticker,
            order_side,
            price,
            55.0,
            source_exchange_2,
        ).unwrap();

        price_level.add_order(order_1);
        price_level.add_order(order_2);
        price_level.add_order(order_3);

        let total_volume = price_level.total_volume();
        let expected_total_volume = (
            NotNan::new(price).unwrap(),
            NotNan::new(20.0 + 25.0 + 55.0).unwrap(),
        );

        assert_eq!(total_volume, expected_total_volume);

        price_level.clear();

        let total_volume = price_level.total_volume();
        let expected_total_volume = (
            NotNan::new(price).unwrap(),
            NotNan::default(),
        );

        assert_eq!(total_volume, expected_total_volume)
    }


    #[test]
    fn single_side_limit_order_book_test() {

        let order_side = OrderSide::BUY;
        let ticker = "EXAMPLE1";

        let mut single_side_limit_order_book = SingleSideLimitOrderBook::new(order_side);

        let source_exchange_1 = "SRCEX1";
        let source_exchange_2 = "SRCEX2";

        let order_1 = Order::new(
            ticker,
            order_side,
            100.0,
            20.0,
            source_exchange_1,
        ).unwrap();

        let order_2 = Order::new(
            ticker,
            order_side,
            102.0,
            10.0,
            source_exchange_2,
        ).unwrap();

        let order_3 = Order::new(
            ticker,
            order_side,
            102.0,
            12.0,
            source_exchange_2,
        ).unwrap();
        
        single_side_limit_order_book.add_order(order_1);
        single_side_limit_order_book.add_order(order_2);
        single_side_limit_order_book.add_order(order_3);

        let total_volume = single_side_limit_order_book.total_volume();
        let expected_total_volume = BTreeMap::from(
            [
                (NotNan::new(100.0).unwrap(), NotNan::new(20.0).unwrap()),
                (NotNan::new(102.0).unwrap(), NotNan::new(10.0 + 12.0).unwrap()),
            ]
        );

        assert_eq!(total_volume, expected_total_volume);
    }


    #[test]
    fn double_side_limit_order_book_test() {

        let ticker = "EXAMPLE1";

        let mut double_side_limit_order_book = DoubleSideLimitOrderBook::new(ticker);

        let source_exchange_1 = "SRCEX1";
        let source_exchange_2 = "SRCEX2";

        let order_1 = Order::new(
            ticker,
            OrderSide::BUY,
            100.0,
            20.0,
            source_exchange_1,
        ).unwrap();

        let order_2 = Order::new(
            ticker,
            OrderSide::BUY,
            102.0,
            10.0,
            source_exchange_2,
        ).unwrap();

        let order_3 = Order::new(
            ticker,
            OrderSide::SELL,
            110.0,
            12.0,
            source_exchange_2,
        ).unwrap();
        
        double_side_limit_order_book.add_order(order_1);
        double_side_limit_order_book.add_order(order_2);
        double_side_limit_order_book.add_order(order_3);

        {
            let total_volume = double_side_limit_order_book.total_volume_buy();
            let expected_total_volume = BTreeMap::from(
                [
                    (NotNan::new(100.0).unwrap(), NotNan::new(20.0).unwrap()),
                    (NotNan::new(102.0).unwrap(), NotNan::new(10.0).unwrap()),
                ]
            );

            assert_eq!(total_volume, expected_total_volume);
        }

        {
            let total_volume = double_side_limit_order_book.total_volume_sell();

            let expected_total_volume = BTreeMap::from(
                [
                    (NotNan::new(110.0).unwrap(), NotNan::new(12.0).unwrap()),
                ]
            );

            assert_eq!(total_volume, expected_total_volume);
        }
    }
}
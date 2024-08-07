
use super::*;

use std::collections::BTreeMap;


const TICKER_1: &str = "EXAMPLE1";
const TICKER_2: &str = "EXAMPLE2";

const SOURCE_EXCHANGE_1: &str = "SRCEX1";
const SOURCE_EXCHANGE_2: &str = "SRCEX2";


#[test]
fn price_level_test() {
    
    let price = 100.0;
    let mut price_level = PriceLevel::new(NotNan::new(price).unwrap());

    let ticker = "EXAMPLE";
    let order_side = OrderSide::BUY;

    let order_1 = Order::new(
        ticker,
        order_side,
        price,
        20.0,
        SOURCE_EXCHANGE_1,
    ).unwrap();

    let order_2 = Order::new(
        ticker,
        order_side,
        price,
        25.0,
        SOURCE_EXCHANGE_1,
    ).unwrap();

    let order_3 = Order::new(
        ticker,
        order_side,
        price,
        55.0,
        SOURCE_EXCHANGE_2,
    ).unwrap();

    price_level.add_order(order_1);
    price_level.add_order(order_2);
    price_level.add_order(order_3);

    let total_volume = price_level.total_volume_with_price_level();
    let expected_total_volume = (
        NotNan::new(price).unwrap(),
        NotNan::new(20.0 + 25.0 + 55.0).unwrap(),
    );

    assert_eq!(total_volume, expected_total_volume);

    price_level.clear();

    let total_volume = price_level.total_volume_with_price_level();
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

    let order_1 = Order::new(
        ticker,
        order_side,
        100.0,
        20.0,
        SOURCE_EXCHANGE_1,
    ).unwrap();

    let order_2 = Order::new(
        ticker,
        order_side,
        102.0,
        10.0,
        SOURCE_EXCHANGE_2,
    ).unwrap();

    let order_3 = Order::new(
        ticker,
        order_side,
        102.0,
        12.0,
        SOURCE_EXCHANGE_2,
    ).unwrap();
    
    single_side_limit_order_book.add_order(order_1);
    single_side_limit_order_book.add_order(order_2);
    single_side_limit_order_book.add_order(order_3);

    let total_volume = single_side_limit_order_book.total_volume_by_price_level();
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

    let order_1 = Order::new(
        ticker,
        OrderSide::BUY,
        100.0,
        20.0,
        SOURCE_EXCHANGE_1,
    ).unwrap();

    let order_2 = Order::new(
        ticker,
        OrderSide::BUY,
        102.0,
        10.0,
        SOURCE_EXCHANGE_2,
    ).unwrap();

    let order_3 = Order::new(
        ticker,
        OrderSide::SELL,
        110.0,
        12.0,
        SOURCE_EXCHANGE_2,
    ).unwrap();
    
    double_side_limit_order_book.add_order(order_1);
    double_side_limit_order_book.add_order(order_2);
    double_side_limit_order_book.add_order(order_3);

    {
        let total_volume = double_side_limit_order_book.total_volume_by_price_level(&OrderSide::BUY);
        let expected_total_volume = BTreeMap::from(
            [
                (NotNan::new(100.0).unwrap(), NotNan::new(20.0).unwrap()),
                (NotNan::new(102.0).unwrap(), NotNan::new(10.0).unwrap()),
            ]
        );

        assert_eq!(total_volume, expected_total_volume);
    }

    {
        let total_volume = double_side_limit_order_book.total_volume_by_price_level(&OrderSide::SELL);

        let expected_total_volume = BTreeMap::from(
            [
                (NotNan::new(110.0).unwrap(), NotNan::new(12.0).unwrap()),
            ]
        );

        assert_eq!(total_volume, expected_total_volume);
    }
}


#[test]
fn multi_ticker_limit_order_book_test() {

    let mut multi_ticker_limit_order_book = MultiTickerLimitOrderBook::new();
    
    let order_1 = Order::new(
        TICKER_1,
        OrderSide::BUY,
        100.0,
        20.0,
        SOURCE_EXCHANGE_1,
    ).unwrap();

    let order_2 = Order::new(
        TICKER_1,
        OrderSide::BUY,
        102.0,
        10.0,
        SOURCE_EXCHANGE_2,
    ).unwrap();

    let order_3 = Order::new(
        TICKER_1,
        OrderSide::SELL,
        110.0,
        12.0,
        SOURCE_EXCHANGE_2,
    ).unwrap();

    let order_4 = Order::new(
        TICKER_2,
        OrderSide::BUY,
        10.0,
        1.0,
        SOURCE_EXCHANGE_1,
    ).unwrap();

    let order_5 = Order::new(
        TICKER_2,
        OrderSide::BUY,
        10.2,
        2.0,
        SOURCE_EXCHANGE_1,
    ).unwrap();

    let order_6 = Order::new(
        TICKER_2,
        OrderSide::BUY,
        10.3,
        3.0,
        SOURCE_EXCHANGE_2,
    ).unwrap();
    
    multi_ticker_limit_order_book.add_order(order_1);
    multi_ticker_limit_order_book.add_order(order_2);
    multi_ticker_limit_order_book.add_order(order_3);
    multi_ticker_limit_order_book.add_order(order_4);
    multi_ticker_limit_order_book.add_order(order_5);
    multi_ticker_limit_order_book.add_order(order_6);

    {
        let total_volume = multi_ticker_limit_order_book.total_volume_by_price_level(TICKER_1, &OrderSide::BUY);
        let expected_total_volume = BTreeMap::from(
            [
                (NotNan::new(100.0).unwrap(), NotNan::new(20.0).unwrap()),
                (NotNan::new(102.0).unwrap(), NotNan::new(10.0).unwrap()),
            ]
        );

        assert_eq!(total_volume, expected_total_volume);
    }

    {
        let total_volume = multi_ticker_limit_order_book.total_volume_by_price_level(TICKER_1, &OrderSide::SELL);

        let expected_total_volume = BTreeMap::from(
            [
                (NotNan::new(110.0).unwrap(), NotNan::new(12.0).unwrap()),
            ]
        );

        assert_eq!(total_volume, expected_total_volume);
    }

    {
        let total_volume = multi_ticker_limit_order_book.total_volume_by_price_level(TICKER_2, &OrderSide::BUY);

        let expected_total_volume = BTreeMap::from(
            [
                (NotNan::new(10.0).unwrap(), NotNan::new(1.0).unwrap()),
                (NotNan::new(10.2).unwrap(), NotNan::new(2.0).unwrap()),
                (NotNan::new(10.3).unwrap(), NotNan::new(3.0).unwrap()),
            ]
        );

        assert_eq!(total_volume, expected_total_volume);
    }

    {
        let total_volume = multi_ticker_limit_order_book.total_volume_by_price_level(TICKER_2, &OrderSide::SELL);

        let expected_total_volume = BTreeMap::from([]);

        assert_eq!(total_volume, expected_total_volume);
    }
}


fn add_some_orders(multi_ticker_limit_order_book: &mut MultiTickerLimitOrderBook) {
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY , 100.0, 20.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY , 100.0, 10.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY , 100.0,  5.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY ,  98.0, 20.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY ,  98.0,  5.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::SELL, 120.0, 20.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::SELL, 120.0, 10.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::SELL, 122.0, 10.0, SOURCE_EXCHANGE_1).unwrap());
    
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY , 100.0, 10.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY , 100.0,  5.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY , 100.0,  2.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY ,  99.0,  5.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::BUY ,  99.0,  5.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::SELL, 120.0, 10.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::SELL, 120.0, 10.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_1, OrderSide::SELL, 120.0,  1.0, SOURCE_EXCHANGE_2).unwrap());

    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::BUY , 1000.0, 20.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::BUY , 1000.0,  5.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::BUY ,  980.0,  5.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::SELL, 1200.0, 20.0, SOURCE_EXCHANGE_1).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::SELL, 1220.0, 10.0, SOURCE_EXCHANGE_1).unwrap());
    
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::BUY , 1000.0, 10.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::BUY , 1000.0,  5.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::BUY ,  990.0,  5.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::BUY ,  990.0,  5.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::SELL, 1200.0, 10.0, SOURCE_EXCHANGE_2).unwrap());
    multi_ticker_limit_order_book.add_order(Order::new(TICKER_2, OrderSide::SELL, 1200.0, 10.0, SOURCE_EXCHANGE_2).unwrap());
}


#[test]
fn multi_ticker_limit_order_book_total_volume_by_price_level_and_source_exchange_test() {

    let mut multi_ticker_limit_order_book = MultiTickerLimitOrderBook::new();

    add_some_orders(&mut multi_ticker_limit_order_book);

    let total_volume_by_price_level_and_source_exchange =
        multi_ticker_limit_order_book.total_volume_by_price_level_and_source_exchange(TICKER_1, &OrderSide::BUY);

    let expected_total_volume_by_price_level_and_source_exchange = BTreeMap::from(
        [
            (
                NotNan::new(98.0).unwrap(),
                BTreeMap::from(
                    [
                        (SOURCE_EXCHANGE_1, NotNan::new(25.0).unwrap()),
                    ]
                )
            ),
            (
                NotNan::new(99.0).unwrap(),
                BTreeMap::from(
                    [
                        (SOURCE_EXCHANGE_2, NotNan::new(10.0).unwrap()),
                    ]
                )
            ),
            (
                NotNan::new(100.0).unwrap(),
                BTreeMap::from(
                    [
                        (SOURCE_EXCHANGE_1, NotNan::new(35.0).unwrap()),
                        (SOURCE_EXCHANGE_2, NotNan::new(17.0).unwrap()),
                    ]
                )
            )
        ]
    );

    assert_eq!(
        total_volume_by_price_level_and_source_exchange,
        expected_total_volume_by_price_level_and_source_exchange
    );
}


#[test]
fn multi_ticker_limit_order_book_total_volume_by_price_level_test() {

    let mut multi_ticker_limit_order_book = MultiTickerLimitOrderBook::new();

    add_some_orders(&mut multi_ticker_limit_order_book);

    let total_volume_by_price_level =
        multi_ticker_limit_order_book.total_volume_by_price_level(TICKER_1, &OrderSide::BUY);

    let expected_total_volume_by_price_level = BTreeMap::from(
        [
            (NotNan::new(98.0).unwrap(), NotNan::new(25.0).unwrap()),
            (NotNan::new(99.0).unwrap(), NotNan::new(10.0).unwrap()),
            (NotNan::new(100.0).unwrap(), NotNan::new(35.0 + 17.0).unwrap())
        ]
    );

    assert_eq!(
        total_volume_by_price_level,
        expected_total_volume_by_price_level
    );
}


#[test]
fn multi_ticker_limit_order_book_total_volume_by_source_exchange_test() {

    // Variables:
    //
    // Multiple Source Exchanges
    // Multiple Tickers
    // Multiple Price Levels
    // Multiple Orders per Price Level

    let mut multi_ticker_limit_order_book = MultiTickerLimitOrderBook::new();

    add_some_orders(&mut multi_ticker_limit_order_book);

    {
        let total_volume_by_source_exchange = 
            multi_ticker_limit_order_book.total_volume_by_source_exchange(TICKER_1, &OrderSide::BUY);

        let expected_total_volume_by_source_exchange = BTreeMap::from(
            [
                (SOURCE_EXCHANGE_1, NotNan::new(60.0).unwrap()),
                (SOURCE_EXCHANGE_2, NotNan::new(27.0).unwrap()),
            ]
        );

        assert_eq!(total_volume_by_source_exchange, expected_total_volume_by_source_exchange);
    }

    {
        let total_volume_by_source_exchange = 
            multi_ticker_limit_order_book.total_volume_by_source_exchange(TICKER_1, &OrderSide::SELL);

        let expected_total_volume_by_source_exchange = BTreeMap::from(
            [
                (SOURCE_EXCHANGE_1, NotNan::new(40.0).unwrap()),
                (SOURCE_EXCHANGE_2, NotNan::new(21.0).unwrap()),
            ]
        );

        assert_eq!(total_volume_by_source_exchange, expected_total_volume_by_source_exchange);
    }
}


// NOTE: Simplified version of the same test above, useful for debugging
// #[test]
// fn multi_ticker_limit_order_book_total_volume_by_source_exchange_simple_test() {

//     // Variables:
//     //
//     // Multiple Source Exchanges
//     // Multiple Tickers
//     // Multiple Price Levels
//     // Multiple Orders per Price Level

//     let mut multi_ticker_limit_order_book = MultiTickerLimitOrderBook::new();

//     let ticker_1 = "EXAMPLE1";
//     let ticker_2 = "EXAMPLE2";

//     let source_exchange_1 = "SRCEX1";
//     let source_exchange_2 = "SRCEX2";

//     multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY , 100.0, 20.0, source_exchange_1).unwrap());
//     multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY , 100.0, 10.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY , 100.0,  5.0, source_exchange_1).unwrap());
//     multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY ,  98.0, 20.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY ,  98.0,  5.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::SELL, 120.0, 20.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::SELL, 120.0, 10.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::SELL, 122.0, 10.0, source_exchange_1).unwrap());
    
//     multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY , 100.0, 10.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY , 100.0,  5.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY , 100.0,  2.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY ,  99.0,  5.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::BUY ,  99.0,  5.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::SELL, 120.0, 10.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::SELL, 120.0, 10.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_1, OrderSide::SELL, 120.0,  1.0, source_exchange_2).unwrap());

//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::BUY , 1000.0, 20.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::BUY , 1000.0,  5.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::BUY ,  980.0,  5.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::SELL, 1200.0, 20.0, source_exchange_1).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::SELL, 1220.0, 10.0, source_exchange_1).unwrap());
    
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::BUY , 1000.0, 10.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::BUY , 1000.0,  5.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::BUY ,  990.0,  5.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::BUY ,  990.0,  5.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::SELL, 1200.0, 10.0, source_exchange_2).unwrap());
//     // multi_ticker_limit_order_book.add_order(Order::new(ticker_2, OrderSide::SELL, 1200.0, 10.0, source_exchange_2).unwrap());

//     multi_ticker_limit_order_book.debug_print();

//     let total_volume_by_source_exchange = 
//         multi_ticker_limit_order_book.total_volume_by_source_exchange(ticker_1, &OrderSide::BUY);

//     let expected_total_volume_by_source_exchange = BTreeMap::from(
//         [
//             (source_exchange_1, NotNan::new(50.0).unwrap()),
//             (source_exchange_2, NotNan::new(10.0).unwrap()),
//         ]
//     );

//     assert_eq!(total_volume_by_source_exchange, expected_total_volume_by_source_exchange);
// }
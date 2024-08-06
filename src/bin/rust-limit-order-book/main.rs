


// order book:
//
// key by: symbol (BTC)
//         order side
//         price level
//         queue per price level
//
// order information:
//
// ticker
// order side
// price
// volume
// source exchange (expect a maximum of 1 per price level)

// wss://ws.kraken.com/v2
// wss://ws-feed.exchange.coinbase.com

// NOTE: Why floats (`f64`) for price?
//
// While it is true that we cannot expect to perform math operations on floating point
// numbers and obtain exact results, eg
//
//     1.0 + 2.0 != 3.0
//
// we *can* expect to consistently parse string representations of floats and obtain
// the same values.
//
// Since this entire codebase only performs string to floating point conversion
// operations when working with prices, and the floating point values are always of
// the same precision (`f64`) this is ok.
//
// NotNan<f64> provides a convenient way of recovering total ordering behaviour of
// floats.
//
// The alternative would be to use a library which implements fixed point numbers
// or Decimal types.


pub mod coinbase_lib;
pub mod gemini_lib;
pub mod kraken_lib;
pub mod limit_order_book_lib;

use coinbase_lib::CoinbaseBookL2;
use gemini_lib::GeminiBook;
use kraken_lib::KrakenBookAPIData;

use reqwest;
use reqwest::header::USER_AGENT;

use std::io::Write;


use ordered_float::NotNan;

use limit_order_book_lib::MultiTickerLimitOrderBook;
use limit_order_book_lib::OrderSide;

fn profit_function_sell<'s, 'l>(
    limit_order_book: &'l mut MultiTickerLimitOrderBook<'s>,
    ticker: &'s str,
    mut target_volume: NotNan<f64>,
) -> f64 {
    let total_volume = limit_order_book.total_volume(ticker, &OrderSide::BUY);

    let mut total_cost = 0.0;

    for (price, volume) in total_volume.iter().rev() {
        //println!("in profit_function_sell: inspecting price level {price}, which has volume {volume}, these should be descending");
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

fn cost_function_buy<'s, 'l>(
    limit_order_book: &'l mut MultiTickerLimitOrderBook<'s>,
    ticker: &'s str,
    mut target_volume: NotNan<f64>,
) -> f64 {
    let total_volume = limit_order_book.total_volume(ticker, &OrderSide::SELL);

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



fn main() {
    println!("Program start");

    use limit_order_book_lib::MultiTickerLimitOrderBook;
    use limit_order_book_lib::Order;
    use limit_order_book_lib::OrderSide;

    let mut limit_order_book = MultiTickerLimitOrderBook::new();
    
    #[allow(non_snake_case)]
    let ticker_BTC_USD = "BTCUSD";
    let source_exchange_coinbase = "COINBASE";
    let source_exchange_gemini = "GEMINI";
    let source_exchange_kraken = "KRAKEN";

    let client = reqwest::blocking::Client::new();

    // Coinbase
    let product_id = "BTC-USD";
    let url_coinbase = format!("https://api.exchange.coinbase.com/products/{product_id}/book?level=2");
    println!("Coinbase URL: {url_coinbase}");

    // Gemini
    let symbol = "btcusd";
    let url_gemini = format!("https://api.gemini.com/v1/book/{symbol}");
    println!("Gemini URL: {url_gemini}");

    // Kraken
    let kraken_pair = "BTCUSD"; // also XXBTZUSD
    let url_kraken = format!("https://api.kraken.com/0/public/Depth?pair={kraken_pair}");

    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36";

    // Gemini
    let response_gemini = client
        .get(url_gemini)
        .header("Content-Type", "application/json")
        .header(USER_AGENT, user_agent)
        .send()
        .expect("client failed to send request");
    println!("Gemini Response Status Code: {}", response_gemini.status());
    
    let response_gemini_text = response_gemini.text().expect("failed to convert response to text");

    {
        let filename = "gemini_btcusd.json";
        let mut file = std::fs::File::create(filename).unwrap_or_else(|error| panic!("failed to create file {filename}: {error}"));
        file.write(response_gemini_text.as_bytes()).expect("failed to write data to file");
    }

    {
        let gemini_book =
            serde_json::from_str::<GeminiBook>(&response_gemini_text)
            .expect("failed to deserialize GeminiBook");

        // println!("deserialized:");
        // let bids = gemini_book.bids.clone();
        // println!("bids:");
        // for bid in bids.into_iter().take(5) {
        //     let GeminiBidAsk{
        //         price,
        //         amount,
        //         timestamp,
        //     } = bid;
        //     println!("{price}, {amount}, {timestamp}");
        // }

        for bid in gemini_book.bids {
            let price = bid.price;
            let volume = bid.amount;
            //println!("adding BID: {price} {volume}");
            let order = Order::new(
                ticker_BTC_USD,
                OrderSide::BUY,
                price,
                volume,
                source_exchange_gemini,
            ).expect("failed to construct Order");
            limit_order_book.add_order(order);
        }

        for ask in gemini_book.asks {
            let price = ask.price;
            let volume = ask.amount;
            //println!("adding ASK: {price} {volume}");
            let order = Order::new(
                ticker_BTC_USD,
                OrderSide::SELL,
                price,
                volume,
                source_exchange_gemini,
            ).expect("failed to construct Order");
            limit_order_book.add_order(order);
        }
    }

    // Kraken

    let response_kraken =
        client
        .get(url_kraken)
        .header("Content-Type", "application/json")
        .header(USER_AGENT, user_agent)
        .send()
        .expect("client failed to send request");
    println!("Kraken Response Status Code: {}", response_kraken.status());
    
    let response_kraken_text = response_kraken.text().expect("failed to convert response to text");

    {
        let filename = "kraken-XXBTZUSD.json";
        let mut file = std::fs::File::create(filename).unwrap_or_else(|error| panic!("failed to create file {filename}: {error}"));
        file.write(response_kraken_text.as_bytes()).expect("failed to write data to file");
    }

    {
        let kraken_book =
            serde_json::from_str::<KrakenBookAPIData>(&response_kraken_text)
            .expect("failed to deserialize KrakenBook");

        for (pair, book) in kraken_book.result {
            if pair != "XXBTZUSD" {
                println!("skipping {pair}");
                continue;
            }

            for bid in book.bids {
                let price = bid.price;
                let volume = bid.volume;
                //println!("adding BID: {price} {volume}");
                let order = Order::new(
                    ticker_BTC_USD,
                    OrderSide::BUY,
                    price,
                    volume,
                    source_exchange_kraken,
                ).expect("failed to construct Order");
                limit_order_book.add_order(order);
            }

            for ask in book.asks {
                let price = ask.price;
                let volume = ask.volume;
                let order = Order::new(
                    ticker_BTC_USD,
                    OrderSide::SELL,
                    price,
                    volume,
                    source_exchange_kraken,
                ).expect("failed to construct Order");
                limit_order_book.add_order(order);
            }
        }
    }

    // Coinbase

    // The behaviour I want here is for the program to panic if this call fails, hence `unwrap`
    let response = client
        .get(url_coinbase)
        .header("Content-Type", "application/json")
        .header(USER_AGENT, user_agent)
        .send()
        .expect("client failed to send request");
    println!("Coinbase Response Status Code: {}", response.status());
    
    let response_text = response.text().expect("failed to convert response to text");

    {
        let filename = "coinbase_BTC-USD.json";
        let mut file = std::fs::File::create(filename).unwrap_or_else(|error| panic!("failed to create file {filename}: {error}"));
        file.write(response_text.as_bytes()).expect("failed to write data to file");
    }

    {
        let coinbase_book =
            serde_json::from_str::<CoinbaseBookL2>(&response_text)
            .expect("failed to deserialize CoinbaseBookL2");

        // println!("deserialized:");
        // let time = coinbase_book.time.clone();
        // println!("time: {time}");
        // let bids = coinbase_book.bids.clone();
        // println!("bids:");
        // for bid in bids.into_iter().take(5) {
        //     let CoinbaseBidAskL2{
        //         price,
        //         volume,
        //         count,
        //     } = bid;
        //     println!("{price}, {volume}, {count}");
        // }

        let filename = "tmp_coinbase_BTC-USE.json";
        let mut file = std::fs::File::create(filename).unwrap();
        file.write(format!("{coinbase_book}").as_bytes()).expect("failed to write data to file");

        for bid in coinbase_book.bids {
            let price = bid.price;
            let volume = bid.volume;
            //println!("adding BID: {price} {volume}");
            let order = Order::new(
                ticker_BTC_USD,
                OrderSide::BUY,
                price,
                volume,
                source_exchange_coinbase,
            ).expect("failed to construct Order");
            limit_order_book.add_order(order);
        }

        for ask in coinbase_book.asks {
            let price = ask.price;
            let volume = ask.volume;
            let order = Order::new(
                ticker_BTC_USD,
                OrderSide::SELL,
                price,
                volume,
                source_exchange_coinbase,
            ).expect("failed to construct Order");
            limit_order_book.add_order(order);
        }
    }

    // println!("highest price level in buy side:");
    // let hbpl = limit_order_book.highest_buy_price_level(ticker_BTC_USD);
    // println!("{hbpl:?}");
    // println!("lowest price level in buy side:");
    // let lbpl = limit_order_book.lowest_buy_price_level(ticker_BTC_USD);
    // println!("{lbpl:?}");

    let target_volume = NotNan::new(10.0).unwrap();

    let total_cost_to_buy = cost_function_buy(&mut limit_order_book, ticker_BTC_USD, target_volume.clone());
    println!("Total cost to BUY {target_volume} BTC: ${total_cost_to_buy}");

    let total_profit_from_sell = profit_function_sell(&mut limit_order_book, ticker_BTC_USD, target_volume.clone());
    println!("Total profit from SELL {target_volume} BTC: ${total_profit_from_sell}");

    println!("Program ends");
}

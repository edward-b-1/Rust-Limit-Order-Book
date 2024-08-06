


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
pub mod limit_order_book_lib;

use coinbase_lib::CoinbaseBookL2;
use coinbase_lib::CoinbaseBidAskL2;

use gemini_lib::GeminiBook;
use gemini_lib::GeminiBidAsk;

use reqwest;
use reqwest::header::USER_AGENT;

use std::io::Write;

use std::collections::VecDeque;


fn main() {
    println!("Program start");

    let client = reqwest::blocking::Client::new();

    // Coinbase
    let product_id = "BTC-USD";
    let url_coinbase = format!("https://api.exchange.coinbase.com/products/{product_id}/book?level=2");
    println!("Coinbase URL: {url_coinbase}");

    // Gemini
    let symbol = "btcusd";
    let url_gemini = format!("https://api.gemini.com/v1/book/{symbol}");
    println!("Gemini URL: {url_gemini}");

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
        let deserialized =
            serde_json::from_str::<GeminiBook>(&response_gemini_text)
            .expect("failed to deserialize");

        println!("deserialized:");
        let bids = deserialized.bids.clone();
        println!("bids:");
        for bid in bids.into_iter().take(5) {
            let GeminiBidAsk{
                price,
                amount,
                timestamp,
            } = bid;
            println!("{price}, {amount}, {timestamp}");
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
        let deserialized =
            serde_json::from_str::<CoinbaseBookL2>(&response_text)
            .expect("failed to deserialize");

        println!("deserialized:");
        let time = deserialized.time.clone();
        println!("time: {time}");
        let bids = deserialized.bids.clone();
        println!("bids:");
        for bid in bids.into_iter().take(5) {
            let CoinbaseBidAskL2{
                price,
                volume,
                count,
            } = bid;
            println!("{price}, {volume}, {count}");
        }

        let filename = "tmp_coinbase_BTC-USE.json";
        let mut file = std::fs::File::create(filename).unwrap();
        file.write(format!("{deserialized}").as_bytes()).expect("failed to write data to file");
    }


    // testing the API for price level
    let source_exchange_coinbase = "COINBASE";
    let source_exchange_gemini = "GEMINI";
    let ticker = "example ticker";

    let order1 = limit_order_book_lib::Order::new(
        ticker,
        limit_order_book_lib::OrderSide::BUY,
        100.0,
        20.0,
        source_exchange_coinbase,
    ).unwrap();

    let order2 = limit_order_book_lib::Order::new(
        ticker,
        limit_order_book_lib::OrderSide::BUY,
        100.0,
        10.0,
        source_exchange_coinbase,
    ).unwrap();

    let order3 = limit_order_book_lib::Order::new(
        ticker,
        limit_order_book_lib::OrderSide::BUY,
        100.0,
        5.0,
        source_exchange_gemini,
    ).unwrap();

    let orders = VecDeque::from(
        vec![order1, order2, order3],
    );

    use ordered_float::NotNan;
    let mut price_level = limit_order_book_lib::PriceLevel::new(NotNan::new(100.0).unwrap());
    for order in orders {
        price_level.add_order(order);
    }


    println!("{price_level:?}");
    

    println!("Program ends");
}

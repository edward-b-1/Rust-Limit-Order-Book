
use std::io::Write;
use std::str::FromStr;
use std::fmt;
use std::fmt::Display;

use serde::Serialize;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;

use reqwest;
use reqwest::header::USER_AGENT;

use crate::limit_order_book_lib::MultiTickerLimitOrderBook;
use crate::limit_order_book_lib::OrderSide;
use crate::limit_order_book_lib::Order;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinbaseBidAskL2 {
    #[serde(deserialize_with="de_from_str")]
    pub price: f64,
    #[serde(deserialize_with="de_from_str")]
    pub volume: f64,
    pub count: u64,
}

impl Display for CoinbaseBidAskL2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let CoinbaseBidAskL2{
            ref price,
            ref volume,
            ref count,
        } = self;
        write!(f, "[{price}, {volume}, {count}]")
    }
}

fn de_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = <&str>::deserialize(deserializer)?;
    f64::from_str(s).map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinbaseBookL2 {
    pub asks: Vec<CoinbaseBidAskL2>,
    pub bids: Vec<CoinbaseBidAskL2>,
    pub time: chrono::DateTime<chrono::offset::Utc>,
}

impl Display for CoinbaseBookL2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let CoinbaseBookL2 {
            ref asks,
            ref bids,
            ref time,
        } = self;
        write!(f, "{{{asks:?}, {bids:?}, {time}}}")
    }
}

pub fn get_api_coinbase(
    client: &reqwest::blocking::Client,
    user_agent: &str,
) -> String {

    // Coinbase
    let product_id = "BTC-USD";
    let url_coinbase = format!("https://api.exchange.coinbase.com/products/{product_id}/book?level=2");
    println!("Coinbase URL: {url_coinbase}");

    // The behaviour I want here is for the program to panic if this call fails, hence `expect`
    let response = client
        .get(url_coinbase)
        .header("Content-Type", "application/json")
        .header(USER_AGENT, user_agent)
        .send()
        .expect("client failed to send request");
    println!("Coinbase Response Status Code: {}", response.status());
    
    let response_text = response.text().expect("failed to convert response to text");
    response_text
}

pub fn dump_coinbase_response_text_to_file(response_text: &str) {
    let filename = "coinbase_BTC-USD.json";
    let mut file = std::fs::File::create(filename).unwrap_or_else(|error| panic!("failed to create file {filename}: {error}"));
    file.write(response_text.as_bytes()).expect("failed to write data to file");
}

pub fn load_coinbase_book<'s>(
    response_text: String,
    ticker: &'s str,
    source_exchange: &'s str,
    limit_order_book: &mut MultiTickerLimitOrderBook<'s>,
) {
    let coinbase_book =
        serde_json::from_str::<CoinbaseBookL2>(&response_text)
        .expect("failed to deserialize CoinbaseBookL2");

    let filename = "tmp_coinbase_BTC-USE.json";
    let mut file = std::fs::File::create(filename).unwrap();
    file.write(format!("{coinbase_book}").as_bytes()).expect("failed to write data to file");

    for bid in coinbase_book.bids {
        let price = bid.price;
        let volume = bid.volume;
        let order = Order::new(
            ticker,
            OrderSide::BUY,
            price,
            volume,
            source_exchange,
        ).expect("failed to construct Order");
        limit_order_book.add_order(order);
    }

    for ask in coinbase_book.asks {
        let price = ask.price;
        let volume = ask.volume;
        let order = Order::new(
            ticker,
            OrderSide::SELL,
            price,
            volume,
            source_exchange,
        ).expect("failed to construct Order");
        limit_order_book.add_order(order);
    }
}
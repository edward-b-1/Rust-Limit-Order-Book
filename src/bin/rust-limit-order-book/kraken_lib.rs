
use std::io::Write;
use std::str::FromStr;
use std::fmt;
use std::fmt::Display;

use std::collections::BTreeMap;

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
pub struct KrakenBidAsk {
    #[serde(deserialize_with="de_from_str")]
    pub price: f64,
    #[serde(deserialize_with="de_from_str")]
    pub volume: f64,
    pub timestamp: u64,
}

impl Display for KrakenBidAsk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let KrakenBidAsk{
            ref price,
            ref volume,
            ref timestamp,
        } = self;
        write!(f, "[{price}, {volume}, {timestamp}]")
    }
}

fn de_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = <&str>::deserialize(deserializer)?;
    f64::from_str(s).map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KrakenBook {
    pub asks: Vec<KrakenBidAsk>,
    pub bids: Vec<KrakenBidAsk>,
}

impl Display for KrakenBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let KrakenBook {
            ref asks,
            ref bids,
        } = self;
        write!(f, "{{{asks:?}, {bids:?}}}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KrakenBookAPIData {
    pub error: Vec<String>,
    pub result: BTreeMap<String, KrakenBook>,
}

impl Display for KrakenBookAPIData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = &self.error;
        let result = &self.result;
        write!(f, "{{\"error\":{error:?},\"result\":{result:?}}}")
    }
}

pub fn get_api_kraken(
    client: &reqwest::blocking::Client,
    user_agent: &str,
) -> String {

    // Kraken
    let kraken_pair = "BTCUSD"; // also XXBTZUSD
    let url_kraken = format!("https://api.kraken.com/0/public/Depth?pair={kraken_pair}");

    // The behaviour I want here is for the program to panic if this call fails, hence `expect`
    let response =
        client
        .get(url_kraken)
        .header("Content-Type", "application/json")
        .header(USER_AGENT, user_agent)
        .send()
        .expect("client failed to send request");
    println!("Kraken Response Status Code: {}", response.status());
    
    let response_text = response.text().expect("failed to convert response to text");
    response_text
}

pub fn dump_kraken_response_text_to_file(response_text: &str) {
    let filename = "kraken-XXBTZUSD.json";
    let mut file = std::fs::File::create(filename).unwrap_or_else(|error| panic!("failed to create file {filename}: {error}"));
    file.write(response_text.as_bytes()).expect("failed to write data to file");
}

pub fn load_kraken_book<'s>(
    response_text: String,
    ticker: &'s str,
    source_exchange: &'s str,
    limit_order_book: &mut MultiTickerLimitOrderBook<'s>,
) {
    let kraken_book =
        serde_json::from_str::<KrakenBookAPIData>(&response_text)
        .expect("failed to deserialize KrakenBook");

    for (pair, book) in kraken_book.result {
        if pair != "XXBTZUSD" {
            println!("skipping {pair}");
            continue;
        }

        for bid in book.bids {
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

        for ask in book.asks {
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
}

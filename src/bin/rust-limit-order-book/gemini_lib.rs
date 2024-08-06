
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
pub struct GeminiBidAsk {
    #[serde(deserialize_with="de_from_str")]
    pub price: f64,
    #[serde(deserialize_with="de_from_str")]
    pub amount: f64,
    #[serde(deserialize_with="de_from_str_u64")]
    pub timestamp: u64,
}

impl Display for GeminiBidAsk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let GeminiBidAsk{
            ref price,
            ref amount,
            ref timestamp,
        } = self;
        write!(f, "[{price}, {amount}, {timestamp}]")
    }
}

fn de_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = <&str>::deserialize(deserializer)?;
    f64::from_str(s).map_err(de::Error::custom)
}

fn de_from_str_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where D: Deserializer<'de>
{
    let s = <&str>::deserialize(deserializer)?;
    u64::from_str(s).map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiBook {
    pub asks: Vec<GeminiBidAsk>,
    pub bids: Vec<GeminiBidAsk>,
}

impl Display for GeminiBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let GeminiBook {
            ref asks,
            ref bids,
        } = self;
        write!(f, "{{{asks:?}, {bids:?}}}")
    }
}

pub fn get_api_gemini(
    client: &reqwest::blocking::Client,
    user_agent: &str,
) -> String {

    // Gemini
    let symbol = "btcusd";
    let url_gemini = format!("https://api.gemini.com/v1/book/{symbol}");
    println!("Gemini URL: {url_gemini}");

    // The behaviour I want here is for the program to panic if this call fails, hence `expect`
    let response = client
        .get(url_gemini)
        .header("Content-Type", "application/json")
        .header(USER_AGENT, user_agent)
        .send()
        .expect("client failed to send request");
    println!("Gemini Response Status Code: {}", response.status());
    
    let response_text = response.text().expect("failed to convert response to text");
    response_text
}

pub fn dump_gemini_response_text_to_file(response_text: &str) {
    let filename = "gemini_btcusd.json";
    let mut file = std::fs::File::create(filename).unwrap_or_else(|error| panic!("failed to create file {filename}: {error}"));
    file.write(response_text.as_bytes()).expect("failed to write data to file");
}

pub fn load_gemini_book<'s>(
    response_text: String,
    ticker: &'s str,
    source_exchange: &'s str,
    limit_order_book: &mut MultiTickerLimitOrderBook<'s>,
) {
    let gemini_book =
        serde_json::from_str::<GeminiBook>(&response_text)
        .expect("failed to deserialize GeminiBook");

    for bid in gemini_book.bids {
        let price = bid.price;
        let volume = bid.amount;
        let order = Order::new(
            ticker,
            OrderSide::BUY,
            price,
            volume,
            source_exchange,
        ).expect("failed to construct Order");
        limit_order_book.add_order(order);
    }

    for ask in gemini_book.asks {
        let price = ask.price;
        let volume = ask.amount;
        //println!("adding ASK: {price} {volume}");
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

use std::str::FromStr;
use std::fmt;
use std::fmt::Display;

use serde::Serialize;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;

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
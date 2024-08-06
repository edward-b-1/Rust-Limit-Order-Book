
use std::str::FromStr;
use std::fmt;
use std::fmt::Display;

use serde::Serialize;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;

use std::collections::BTreeMap;

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

use std::str::FromStr;
use std::fmt;
use std::fmt::Display;

use serde::Serialize;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;

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
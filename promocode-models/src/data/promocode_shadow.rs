use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PromocodeShadow {
    pub _id: String,
    pub name: String,
    pub avantage: AvantageShadow,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<RestrictionShadow>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AvantageShadow {
    pub percent: u8,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum RestrictionShadow {
    #[serde(rename = "@date")]
    Date { after: String, before: String },

    #[serde(rename = "@age")]
    Age {
        #[serde(skip_serializing_if = "Option::is_none")]
        lt: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        eq: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gt: Option<u8>,
    },

    #[serde(rename = "@meteo")]
    Meteo { is: String, temp: TempShadow },

    #[serde(rename = "@and")]
    And(Vec<RestrictionShadow>),

    #[serde(rename = "@or")]
    Or(Vec<RestrictionShadow>),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TempShadow {
    pub gt: String,
}

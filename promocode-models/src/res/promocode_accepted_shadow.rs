use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PromocodeAcceptedShadow {
    pub promocode_name: String,
    pub status: String,
    pub avantage: AvantageShadow,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AvantageShadow {
    pub percent: u8,
}

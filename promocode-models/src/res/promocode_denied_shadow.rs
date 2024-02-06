use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PromocodeDeniedShadow {
    pub promocode_name: String,
    pub status: String,
    pub reasons: ReasonsShadow,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ReasonsShadow {}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PromocodeRequestShadow {
    pub promocode_name: String,
    pub arguments: ArgumentsShadow,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ArgumentsShadow {
    pub age: u8,
    pub meteo: MeteoShadow,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MeteoShadow {
    pub town: String,
}

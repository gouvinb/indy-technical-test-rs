use crate::data::_shadow::AvantageShadow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "AvantageShadow")]
pub struct Avantage {
    pub percent: u8,
}

impl Avantage {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    pub fn validate(&self) -> Result<Avantage, /* Error */ String> {
        // TODO: Ask to product: percent may be 0 ?
        if !(1u8..=100u8).contains(&self.percent) {
            return Err("`percent` must be greater than 0 and lower than 101.".to_string());
        }

        Ok(self.clone())
    }
}

impl TryFrom<AvantageShadow> for Avantage {
    type Error = String;

    fn try_from(value: AvantageShadow) -> Result<Self, Self::Error> {
        // TODO: Ask to product: percent may be 0 ?
        if !(1u8..=100u8).contains(&value.percent) {
            return Err("`percent` must be greater than 0 and lower than 101.".to_string());
        }

        Ok(Avantage { percent: value.percent })
    }
}

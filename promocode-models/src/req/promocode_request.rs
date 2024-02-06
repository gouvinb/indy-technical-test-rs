use serde::{Deserialize, Serialize};

use crate::req::promocode_request_shadow::{ArgumentsShadow, MeteoShadow, PromocodeRequestShadow};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "PromocodeRequestShadow")]
pub struct PromocodeRequest {
    pub promocode_name: String,
    pub arguments: Arguments,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "ArgumentsShadow")]
pub struct Arguments {
    pub age: u8,
    pub meteo: Meteo,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "MeteoShadow")]
pub struct Meteo {
    pub town: String,
}

impl PromocodeRequest {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    pub fn validate(&self) -> Result<PromocodeRequest, /* Error */ String> {
        if self.promocode_name.is_empty() {
            return Err("`promocode_name` must be nonempty.".to_string());
        }

        self.arguments.validate()?;

        Ok(self.clone())
    }
}

impl Arguments {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    pub fn validate(&self) -> Result<Arguments, /* Error */ String> {
        self.meteo.validate()?;

        Ok(self.clone())
    }
}

impl Meteo {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    pub fn validate(&self) -> Result<Meteo, /* Error */ String> {
        if self.town.is_empty() {
            return Err("`town` must be nonempty.".to_string());
        }

        Ok(self.clone())
    }
}

impl TryFrom<PromocodeRequestShadow> for PromocodeRequest {
    type Error = String;

    fn try_from(value: PromocodeRequestShadow) -> Result<Self, Self::Error> {
        if value.promocode_name.is_empty() {
            return Err("`promocode_name` must be nonempty.".to_string());
        }

        let arguments_converted = match Arguments::try_from(value.arguments) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        Ok(PromocodeRequest {
            promocode_name: value.promocode_name,
            arguments: arguments_converted,
        })
    }
}

impl TryFrom<ArgumentsShadow> for Arguments {
    type Error = String;

    fn try_from(value: ArgumentsShadow) -> Result<Self, Self::Error> {
        let meteo_converted = match Meteo::try_from(value.meteo) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        Ok(Arguments {
            age: value.age,
            meteo: meteo_converted,
        })
    }
}

impl TryFrom<MeteoShadow> for Meteo {
    type Error = String;

    fn try_from(value: MeteoShadow) -> Result<Self, Self::Error> {
        if value.town.is_empty() {
            return Err("`town` must be nonempty.".to_string());
        }

        Ok(Meteo { town: value.town })
    }
}

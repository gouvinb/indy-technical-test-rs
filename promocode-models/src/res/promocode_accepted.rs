use serde::{Deserialize, Serialize};

use crate::res::promocode_accepted_shadow::{AvantageShadow, PromocodeAcceptedShadow};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "PromocodeAcceptedShadow")]
pub struct PromocodeAccepted {
    pub promocode_name: String,
    pub status: String,
    pub avantage: Avantage,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "AvantageShadow")]
pub struct Avantage {
    pub percent: u8,
}

impl PromocodeAccepted {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    /// Validates the [PromocodeAccepted] struct.
    ///
    /// # Errors
    ///
    /// Returns an error if [promocode_name] is empty or if there is an error
    /// validating [avantage].
    ///
    /// # Returns
    ///
    /// Returns [Ok] with a [PromocodeAccepted] if the struct is valid.
    pub fn validate(&self) -> Result<PromocodeAccepted, /* Error */ String> {
        if self.promocode_name.is_empty() {
            return Err("`promocode_name` must be nonempty.".to_string());
        }

        self.avantage.validate()?;

        Ok(self.clone())
    }
}

impl Avantage {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    /// Validates the [Avantage].
    ///
    /// # Returns
    ///
    /// The result of the validation. [Ok] with cloned [Avantage] if the
    /// validation is successful, or [Err] with an error message if the
    /// [percent] value is not within the range of 1 to 100 (inclusive).
    pub fn validate(&self) -> Result<Avantage, /* Error */ String> {
        if !(0u8..=100u8).contains(&self.percent) {
            return Err("`percent` must be greater than 0 and lower than 101.".to_string());
        }

        Ok(self.clone())
    }
}

impl TryFrom<PromocodeAcceptedShadow> for PromocodeAccepted {
    type Error = String;

    fn try_from(value: PromocodeAcceptedShadow) -> Result<Self, Self::Error> {
        if value.promocode_name.is_empty() {
            return Err("`promocode_name` must be nonempty.".to_string());
        }

        let avantage_converted = match Avantage::try_from(value.avantage) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        Ok(PromocodeAccepted {
            promocode_name: value.promocode_name,
            status: value.status,
            avantage: avantage_converted,
        })
    }
}

impl TryFrom<AvantageShadow> for Avantage {
    type Error = String;

    fn try_from(value: AvantageShadow) -> Result<Self, Self::Error> {
        if !(0u8..=100u8).contains(&value.percent) {
            return Err("`percent` must be greater than 0 and lower than 101.".to_string());
        }

        Ok(Avantage { percent: value.percent })
    }
}

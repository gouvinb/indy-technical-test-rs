use crate::data::_shadow::TempShadow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "TempShadow")]
pub struct Temp {
    pub gt: String,
}

impl Temp {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    /// Validate the current object.
    ///
    /// # Errors
    ///
    /// Returns a `String` error if any of the following conditions are met:
    /// - The `gt` field is empty.
    /// - The `gt` field cannot be parsed as a `f64`.
    ///
    /// # Returns
    ///
    /// Returns `Ok` with a cloned reference to `Self` if the validation is
    /// successful.
    pub fn validate(&self) -> Result<Self, /* Error */ String> {
        if self.gt.is_empty() {
            return Err("`gt` must be nonempty.".to_string());
        }

        if self.gt.as_str().parse::<f64>().is_err() {
            return Err("`gt` must be an float.".to_string());
        }

        Ok(self.clone())
    }
}

impl TryFrom<TempShadow> for Temp {
    type Error = String;

    fn try_from(value: TempShadow) -> Result<Self, Self::Error> {
        if value.gt.is_empty() {
            return Err("`gt` must be nonempty.".to_string());
        }
        if value.gt.as_str().parse::<f64>().is_err() {
            return Err("`gt` must be an float.".to_string());
        }

        Ok(Temp { gt: value.gt.to_string() })
    }
}

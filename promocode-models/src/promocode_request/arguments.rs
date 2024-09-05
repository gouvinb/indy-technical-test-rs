use crate::promocode_request::meteo::Meteo;
use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct Arguments {
    pub age: u8,
    pub meteo: Meteo,
}

impl Arguments {
    /// Create a new [`Arguments`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if one of field is not correct.
    pub fn new(age: u8, meteo: Result<Meteo, String>) -> Result<Self, String> {
        let meteo = match meteo {
            Err(err_after) => return Err(format!("`meteo` {}", err_after)),
            Ok(value) => value,
        };

        Ok(Self { age, meteo })
    }

    /// Create a new [Arguments] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is unsafe because it doesn't check the validity of the
    /// input parameters. As such, the responsibility of validating these
    /// parameters falls on the caller of this function.
    ///
    /// # Example
    ///
    /// ```
    /// // the input here is assumed to be correct, but no checks are made
    /// // whatsoever
    /// use promocode_models::promocode_request::{
    ///     PromocodeRequest, arguments::Arguments, meteo::Meteo
    /// };
    ///
    /// let arg = unsafe {
    ///     Arguments::new_unchecked(1u8, Meteo::new_unchecked("town".to_string()));
    /// };
    /// ```
    ///
    /// Parameters:
    /// - `age`: A u8 that represents the user's age.
    /// - `meteo`: Some user's position for the meteo restriction.
    pub unsafe fn new_unchecked(age: u8, meteo: Meteo) -> Self {
        Self { age, meteo }
    }
}

impl<'de> Deserialize<'de> for Arguments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
        struct ArgumentsUnsafe {
            age: u8,
            meteo: Meteo,
        }

        match ArgumentsUnsafe::deserialize(deserializer) {
            Ok(data) => Arguments::new(data.age, Ok(data.meteo)).map_err(Error::custom),
            Err(err) => Err(Error::custom(err)),
        }
    }
}

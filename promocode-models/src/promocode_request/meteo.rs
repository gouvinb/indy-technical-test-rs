use promocode_util::validate_type::string::NonBlankString;
use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct Meteo {
    town: NonBlankString,
}

impl Meteo {
    /// Create a new [`Meteo`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if one of field is not correct.
    pub fn new(town: String) -> Result<Self, String> {
        let town = match NonBlankString::new(town) {
            Err(err_id) => {
                return Err(format!("`town` {}", err_id));
            },
            Ok(value) => value,
        };

        Ok(Self { town })
    }

    /// Create a new [Meteo] (unchecked)
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
    /// use promocode_models::promocode_request::meteo::Meteo;
    ///
    /// let meteo = unsafe { Meteo::new_unchecked("town".to_string()) };
    /// ```
    ///
    /// Parameters:
    /// - `town`: A string that represents the user's city.
    pub unsafe fn new_unchecked(town: String) -> Self {
        Self {
            town: NonBlankString::new_unchecked(town),
        }
    }

    /// Returns the town as [String] type
    pub fn town(&self) -> String {
        self.town.clone().get()
    }
}

impl<'de> Deserialize<'de> for Meteo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
        struct MeteoUnsafe {
            town: String,
        }

        match MeteoUnsafe::deserialize(deserializer) {
            Ok(data) => Meteo::new(data.town).map_err(Error::custom),
            Err(err) => Err(Error::custom(err)),
        }
    }
}

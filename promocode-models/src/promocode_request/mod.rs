use arguments::Arguments;
use promocode_util::validate_type::string::NonBlankString;
use serde::{de::Error, Deserialize, Deserializer, Serialize};

pub mod arguments;
pub mod meteo;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct PromocodeRequest {
    promocode_name: NonBlankString,
    pub arguments: Arguments,
}

impl PromocodeRequest {
    /// Create a new [`PromocodeRequest`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if one of field is not correct.
    pub fn new(promocode_name: String, arguments: Arguments) -> Result<Self, String> {
        let promocode_name = match NonBlankString::new(promocode_name) {
            Err(err_id) => {
                return Err(format!("`promocode_name` {}", err_id));
            },
            Ok(value) => value,
        };

        Ok(Self {
            promocode_name,
            arguments,
        })
    }

    /// Create a new [PromocodeRequest] (unchecked)
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
    /// let pcr = unsafe {
    ///     PromocodeRequest::new_unchecked(
    ///         "promocode_name".to_string(),
    ///         Arguments::new_unchecked(1u8, Meteo::new_unchecked("town".to_string())),
    ///     );
    /// };
    /// ```
    ///
    /// Parameters:
    /// - `promocode_name`: A string that represents the name of the promo code.
    /// - `arguments`: Some user information like age and position for the meteo
    ///   restriction.
    pub unsafe fn new_unchecked(promocode_name: String, arguments: Arguments) -> Self {
        Self {
            promocode_name: NonBlankString::new_unchecked(promocode_name),
            arguments,
        }
    }

    /// Returns the promocode_name as [String] type
    pub fn promocode_name(&self) -> String {
        self.promocode_name.clone().get()
    }
}

impl<'de> Deserialize<'de> for PromocodeRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
        struct PromocodeRequestUnsafe {
            promocode_name: String,
            arguments: Arguments,
        }

        match PromocodeRequestUnsafe::deserialize(deserializer) {
            Ok(data) => PromocodeRequest::new(data.promocode_name, data.arguments).map_err(Error::custom),
            Err(err) => Err(Error::custom(err)),
        }
    }
}

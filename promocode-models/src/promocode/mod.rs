use crate::promocode_response::{reason::Reasons, PromocodeResponse};
use avantage::Avantage;
use promocode_util::validate_type::string::NonBlankString;
use restrictions::Restrictions;
use serde::{de::Error, Deserialize, Deserializer, Serialize};

pub mod avantage;
pub mod restriction;
pub mod restrictions;
pub mod temp;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct Promocode {
    _id: NonBlankString,
    name: NonBlankString,
    pub avantage: Avantage,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Restrictions,
}

impl Promocode {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    /// Create a new [`Promocode`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if one of field is not correct.
    pub fn new(_id: String, name: String, avantage: Avantage, restrictions: Restrictions) -> Result<Self, String> {
        let _id = match NonBlankString::new(_id) {
            Err(err_id) => {
                return Err(format!("`_id` {}", err_id));
            },
            Ok(value) => value,
        };

        let name = match NonBlankString::new(name) {
            Err(err_name) => {
                return Err(format!("`name` {}", err_name));
            },
            Ok(value) => value,
        };

        Ok(Self {
            _id,
            name,
            avantage,
            restrictions,
        })
    }

    /// Create a new [Promocode] (unchecked)
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
    /// use promocode_models::promocode::avantage::Avantage;
    /// use promocode_models::promocode::Promocode;
    /// use promocode_models::promocode::restrictions::Restrictions;
    ///
    /// let pc = unsafe {
    ///     Promocode::new_unchecked(
    ///         "_id".to_string(),
    ///         "name".to_string(),
    ///         Avantage::new_unchecked(1u8),
    ///         Restrictions::new()
    ///     );
    /// };
    /// ```
    ///
    /// Parameters:
    /// - `_id`: A string that stands as an unique identifier for the promo code.
    /// - `name`: A string that represents the name of the promo code.
    /// - `avantage`: An `Avantage` instance representing the advantage of the promo code.
    /// - `restrictions`: A list of `Restriction` instances representing the restrictions of the promo code.
    pub unsafe fn new_unchecked(_id: String, name: String, avantage: Avantage, restrictions: Restrictions) -> Self {
        Self {
            _id: NonBlankString::new_unchecked(_id),
            name: NonBlankString::new_unchecked(name),
            avantage,
            restrictions,
        }
    }

    /// Returns the _id as [String] type
    pub fn _id(&self) -> String {
        self._id.clone().get()
    }

    /// Returns the _id as [String] type
    pub fn name(&self) -> String {
        self.name.clone().get()
    }

    /// Generate a response for a given promocode.
    ///
    /// # Arguments
    ///
    /// * `promocode_name` - The name of the promocode.
    /// * `percent` - The percentage discount for the promocode.
    /// * `predicate` - A boolean value indicating whether the promocode is accepted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `PromocodeResponse` if the promocode is accepted or denied,
    /// or an error message as a `String` if the promocode response cannot build correctly.
    ///
    pub fn generate_response(promocode_name: String, percent: u8, predicate: bool) -> Result<PromocodeResponse, String> {
        if predicate {
            PromocodeResponse::accepted(promocode_name, Avantage::new(percent)?)
        } else {
            PromocodeResponse::denied(promocode_name, Reasons {})
        }
    }
}

impl<'de> Deserialize<'de> for Promocode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
        pub struct PromocodeUnsafe {
            _id: String,
            name: String,
            pub avantage: Avantage,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            pub restrictions: Restrictions,
        }

        match PromocodeUnsafe::deserialize(deserializer) {
            Ok(data) => Promocode::new(data._id, data.name, data.avantage, data.restrictions).map_err(Error::custom),
            Err(err) => Err(Error::custom(err)),
        }
    }
}

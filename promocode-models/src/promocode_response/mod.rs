use crate::promocode::avantage::Avantage;
use promocode_util::validate_type::string::NonBlankString;
use reason::Reasons;
use serde::{de::Error, Deserialize, Serialize};
use status::Status;

pub mod reason;
pub mod status;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum PromocodeResponse {
    Accepted {
        promocode_name: NonBlankString,
        status: Status,
        avantage: Avantage,
    },

    Denied {
        promocode_name: NonBlankString,
        status: Status,
        reasons: Reasons,
    },
}

impl PromocodeResponse {
    /// Create a new [`PromocodeResponse::Accepted`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `PromocodeResponse::Accepted` is not correct.
    pub fn accepted(promocode_name: String, avantage: Result<Avantage, String>) -> Result<Self, String> {
        let promocode_name = match NonBlankString::new(promocode_name) {
            Err(err_after) => return Err(format!("`promocode_name` {}", err_after)),
            Ok(value) => value,
        };

        let avantage = match avantage {
            Err(err_after) => return Err(format!("`avantage` > {}", err_after)),
            Ok(value) => value,
        };

        Ok(PromocodeResponse::Accepted {
            promocode_name,
            status: Status::Accepted,
            avantage,
        })
    }

    /// Create a new [PromocodeResponse::Accepted] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it assumes that
    /// `PromocodeResponse::Accepted` is correct, without performing any
    /// validation.
    /// It's up to the caller to ensure any `promocode_name` string passed is
    /// not blank.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use promocode_models::promocode::avantage::Avantage;
    /// use promocode_models::promocode_response::PromocodeResponse;
    ///
    /// let response = unsafe {
    ///     PromocodeResponse::accepted_unchecked(
    ///         "promocode_name".to_string(),
    ///         Avantage::new_unchecked(1u8),
    ///     )
    /// };
    /// ```
    ///
    /// This function assumes `PromocodeResponse::Accepted` is correct.
    pub unsafe fn accepted_unchecked(promocode_name: String, avantage: Avantage) -> Self {
        PromocodeResponse::Accepted {
            promocode_name: NonBlankString::new_unchecked(promocode_name),
            status: Status::Accepted,
            avantage,
        }
    }

    /// Create a new [`PromocodeResponse::Denied`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `PromocodeResponse::Denied` is not correct.
    pub fn denied(promocode_name: String, reasons: Reasons) -> Result<Self, String> {
        let promocode_name = match NonBlankString::new(promocode_name) {
            Err(err_after) => return Err(format!("`promocode_name` {}", err_after)),
            Ok(value) => value,
        };

        Ok(PromocodeResponse::Denied {
            promocode_name,
            status: Status::Denied,
            reasons,
        })
    }

    /// Create a new [PromocodeResponse::Denied] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it assumes that
    /// `PromocodeResponse::Denied` is correct, without performing any
    /// validation.
    /// It's up to the caller to ensure any `promocode_name` string passed is
    /// not blank.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use promocode_models::promocode_response::PromocodeResponse;
    /// use promocode_models::promocode_response::reason::Reasons;
    ///
    /// let response = unsafe {
    ///     PromocodeResponse::denied_unchecked(
    ///         "promocode_name".to_string(),
    ///         Reasons {},
    ///     )
    /// };
    /// ```
    ///
    /// This function assumes `PromocodeResponse::Accepted` is correct.
    pub unsafe fn denied_unchecked(promocode_name: String, reasons: Reasons) -> Self {
        PromocodeResponse::Denied {
            promocode_name: NonBlankString::new_unchecked(promocode_name),
            status: Status::Denied,
            reasons,
        }
    }
}

impl<'de> Deserialize<'de> for PromocodeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
        struct PromocodeResponseUnknown {
            promocode_name: NonBlankString,
            status: Status,
            avantage: Option<Avantage>,
            reasons: Option<Reasons>,
        }

        let data = PromocodeResponseUnknown::deserialize(deserializer)?;

        match (data.status.clone(), data.avantage, data.reasons) {
            (Status::Accepted, Some(avantage), None) => Ok(PromocodeResponse::Accepted {
                promocode_name: data.promocode_name,
                status: data.status,
                avantage,
            }),
            (Status::Denied, None, Some(reasons)) => Ok(PromocodeResponse::Denied {
                promocode_name: data.promocode_name,
                status: data.status,
                reasons,
            }),
            _ => Err(D::Error::custom("Invalid promocode response")),
        }
    }
}

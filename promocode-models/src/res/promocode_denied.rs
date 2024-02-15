use serde::{Deserialize, Serialize};

use crate::res::promocode_denied_shadow::{PromocodeDeniedShadow, ReasonsShadow};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "PromocodeDeniedShadow")]
pub struct PromocodeDenied {
    pub promocode_name: String,
    pub status: String,
    pub reasons: Reasons,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "ReasonsShadow")]
pub struct Reasons {}

impl PromocodeDenied {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    /// Validates the [PromocodeDenied] instance.
    ///
    /// # Errors
    ///
    /// Returns a [Result] containing either a [PromocodeDenied] instance if
    /// validation fails or an error [String].
    pub fn validate(&self) -> Result<PromocodeDenied, /* Error */ String> {
        if self.promocode_name.is_empty() {
            return Err("`promocode_name` must be nonempty.".to_string());
        }

        self.reasons.validate()?;

        Ok(self.clone())
    }
}

impl Reasons {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    /// NOOP: But expect a [Reasons] validation
    pub fn validate(&self) -> Result<Reasons, /* Error */ String> {
        // TODO: Ask to product: what we do here ?
        Ok(self.clone())
    }
}

impl TryFrom<PromocodeDeniedShadow> for PromocodeDenied {
    type Error = String;

    fn try_from(value: PromocodeDeniedShadow) -> Result<Self, Self::Error> {
        if value.promocode_name.is_empty() {
            return Err("`promocode_name` must be nonempty.".to_string());
        }

        let reason_converted = match Reasons::try_from(value.reasons) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        Ok(PromocodeDenied {
            promocode_name: value.promocode_name,
            status: value.status,
            reasons: reason_converted,
        })
    }
}

impl TryFrom<ReasonsShadow> for Reasons {
    type Error = String;

    // TODO: Ask to product: what we do here ?
    #[allow(unused_variables)]
    fn try_from(value: ReasonsShadow) -> Result<Self, Self::Error> {
        Ok(Reasons {})
    }
}

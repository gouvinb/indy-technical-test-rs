/// This module contains the [_shadow] module.
///
/// The [_shadow] module provides helper functions and data models for
/// [Promocode] deserialization validation. The module can be used to handle
/// [Promocode] related operations.
///
/// # Example
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use promocode_models::data::avantage::Avantage;
/// use promocode_models::extensions::vec_restriction::Restrictions;
///
/// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
/// #[serde(try_from = "PromocodeShadow")] // <== Validation with TryFrom trait
/// pub struct Promocode {
///     pub _id: String,
///     pub name: String,
///     pub avantage: Avantage,
///     #[serde(skip_serializing_if = "Vec::is_empty")]
///     pub restrictions: Restrictions,
/// }
/// ```
///
/// # References
///
/// - [serde-rs/serde#939-939514114](https://github.com/serde-rs/serde/issues/939#issuecomment-939514114)
mod _shadow;
pub mod avantage;
pub mod promocode;
pub mod restriction;
pub mod temp;

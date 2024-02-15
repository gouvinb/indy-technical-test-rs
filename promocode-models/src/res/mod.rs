/// This module contains the [promocode_accepted_shadow] module.
///
/// The [promocode_accepted_shadow] module provides helper functions and data
/// models for [PromocodeAccepted] deserialization validation. The module can be
/// used to handle [PromocodeAccepted] related operations.
///
/// # Example
///
/// ```rust
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
/// #[serde(try_from = "AvantageShadow")] // <== Validation with TryFrom trait
/// pub struct Avantage {
///     /*...*/
/// }
/// ```
///
/// # References
///
/// - [serde-rs/serde#939-939514114](https://github.com/serde-rs/serde/issues/939#issuecomment-939514114)
mod promocode_accepted_shadow;
pub mod promocode_accepted;

/// This module contains the [promocode_denied_shadow] module.
///
/// The [promocode_denied_shadow] module provides helper functions and data
/// models for [PromocodeDenied] deserialization validation. The module can be
/// used to handle [PromocodeDenied] related operations.
///
/// # Example
///
/// ```rust
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
/// #[serde(try_from = "ReasonsShadow")] // <== Validation with TryFrom trait
/// pub struct Reasons {
///     /*...*/
/// }
/// ```
///
/// # References
///
/// - [serde-rs/serde#939-939514114](https://github.com/serde-rs/serde/issues/939#issuecomment-939514114)
mod promocode_denied_shadow;
pub mod promocode_denied;

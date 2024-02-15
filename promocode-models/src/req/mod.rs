/// This module contains the [promocode_request_shadow] module.
///
/// The [promocode_request_shadow] module provides helper functions and data
/// models for [PromocodeRequest] deserialization validation. The module can be
/// used to handle [Promocode] related operations.
///
/// # Example
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use promocode_models::req::promocode_request::Arguments;
///
/// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
/// #[serde(try_from = "PromocodeRequestShadow")] // <== Validation with TryFrom trait
/// pub struct PromocodeRequest {
///     pub promocode_name: String,
///     pub arguments: Arguments,
/// }
/// ```
///
/// # References
///
/// - [serde-rs/serde#939-939514114](https://github.com/serde-rs/serde/issues/939#issuecomment-939514114)
mod promocode_request_shadow;
pub mod promocode_request;

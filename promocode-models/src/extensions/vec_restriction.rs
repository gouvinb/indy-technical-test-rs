use crate::data::restriction::Restriction;
use crate::req::promocode_request::Arguments;

/// A collection of `Restriction` objects
pub type Restrictions = Vec<Restriction>;

/// Trait for extending the functionality of `Restrictions`.
pub trait RestrictionsExt {
    fn check_restriction_or(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool;
    fn check_restriction_and(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool;
}

impl RestrictionsExt for Restrictions {
    /// Checks if the request satisfies one of the given [Restrictions]. Returns
    /// a boolean indicating whether the request is valid or not.
    ///
    /// (Implicit [Restriction::Or])
    ///
    /// # Arguments
    ///
    /// - `arguments` - Requested arguments.
    /// - `weather_and_temp` - The optional weather condition and temperature.
    fn check_restriction_or(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool {
        self.iter()
            .any(|restriction| restriction.check_restriction_generic(arguments.clone(), weather_and_temp.clone()))
    }

    /// Checks if the request satisfies all the given [Restrictions]. Returns a
    /// boolean indicating whether the request is valid or not.
    ///
    /// # Arguments
    ///
    /// - `arguments` - Requested arguments.
    /// - `weather_and_temp` - The optional weather condition and temperature.
    fn check_restriction_and(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool {
        self.iter()
            .all(|restriction| restriction.check_restriction_generic(arguments.clone(), weather_and_temp.clone()))
    }
}

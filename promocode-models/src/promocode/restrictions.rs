use crate::{promocode::restriction::Restriction, promocode_request::arguments::Arguments};
use promocode_util::validate_type::sequence::NonEmptyVec;

/// A collection of `Restriction` objects
pub type Restrictions = Vec<Restriction>;

/// Trait for extending the functionality of `Restrictions`.
pub trait RestrictionsExt<T = Self> {
    fn from_vec(value: Vec<Result<Restriction, String>>) -> Result<T, String>;
    fn check_restriction_or(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool;
    fn check_restriction_and(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool;
}

impl RestrictionsExt for Restrictions {
    fn from_vec(value: Vec<Result<Restriction, String>>) -> Result<Restrictions, String> {
        if value.iter().any(|it| it.is_err()) {
            Err(value
                .iter()
                .filter(|it| it.is_err())
                .map(|it| it.clone().unwrap_err())
                .collect::<Vec<String>>()
                .join("\n"))
        } else {
            Ok(value.iter().map(|it| it.clone().unwrap()).collect())
        }
    }

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

pub type SubRestrictions = NonEmptyVec<Restriction>;

impl RestrictionsExt for SubRestrictions {
    fn from_vec(value: Vec<Result<Restriction, String>>) -> Result<SubRestrictions, String> {
        if value.iter().any(|it| it.is_err()) {
            Err(value
                .iter()
                .filter(|it| it.is_err())
                .map(|it| it.clone().unwrap_err())
                .collect::<Vec<String>>()
                .join("\n"))
        } else {
            let sub_restrictions_vec: Vec<Restriction> = value.iter().map(|it| it.clone().unwrap()).collect();
            SubRestrictions::new(sub_restrictions_vec).map_err(|err| err.to_string())
        }
    }

    /// Checks if the request satisfies one of the given [SubRestrictions]. Returns
    /// a boolean indicating whether the request is valid or not.
    ///
    /// (Implicit [Restriction::Or])
    ///
    /// # Arguments
    ///
    /// - `arguments` - Requested arguments.
    /// - `weather_and_temp` - The optional weather condition and temperature.
    fn check_restriction_or(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool {
        self.clone()
            .get()
            .iter()
            .any(|restriction| restriction.check_restriction_generic(arguments.clone(), weather_and_temp.clone()))
    }

    /// Checks if the request satisfies all the given [SubRestrictions]. Returns a
    /// boolean indicating whether the request is valid or not.
    ///
    /// # Arguments
    ///
    /// - `arguments` - Requested arguments.
    /// - `weather_and_temp` - The optional weather condition and temperature.
    fn check_restriction_and(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool {
        self.clone()
            .get()
            .iter()
            .all(|restriction| restriction.check_restriction_generic(arguments.clone(), weather_and_temp.clone()))
    }
}

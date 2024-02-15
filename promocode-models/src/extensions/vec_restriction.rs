use chrono::{NaiveDate, Utc};
use log::error;

use crate::data::restriction::Restriction;
use crate::req::promocode_request::Arguments;

/// A collection of `Restriction` objects
pub type Restrictions = Vec<Restriction>;

/// Trait for extending the functionality of `Restrictions`.
pub trait RestrictionsExt {
    fn check_request(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool;
}

impl RestrictionsExt for Restrictions {
    /// Checks if the request satisfies all the given [Restrictions]. Returns a
    /// boolean indicating whether the request is valid or not.
    ///
    /// # Arguments
    ///
    /// - `arguments` - The arguments of the request.
    /// - `weather_and_temp` - The optional weather condition and temperature.
    fn check_request(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool {
        self.iter().fold(true, |acc_predicate, restriction| match restriction {
            Restriction::Date { after, before } => {
                let now = Utc::now().date_naive();
                let after_date = NaiveDate::parse_from_str(after.as_str(), "%Y-%m-%d").unwrap_or(NaiveDate::MIN);
                let before_date = NaiveDate::parse_from_str(before.as_str(), "%Y-%m-%d").unwrap_or(NaiveDate::MAX);

                acc_predicate && after_date <= now && now <= before_date
            },
            Restriction::Age { lt, eq, gt } => {
                acc_predicate
                    && match (gt, eq, lt) {
                        (None, Some(eq_u8), None) => &arguments.age == eq_u8,
                        (Some(gt_u8), None, None) => &arguments.age >= gt_u8,
                        (None, None, Some(lt_u8)) => &arguments.age <= lt_u8,
                        (Some(gt_u8), None, Some(lt_u8)) => gt_u8 <= &arguments.age && &arguments.age <= lt_u8,
                        _ => false,
                    }
            },
            Restriction::Meteo { is, temp } => {
                acc_predicate
                    && match weather_and_temp {
                        None => {
                            error!("Skip meteo check and return false because open_weather_sdk_unchecked is None.");
                            false
                        },
                        Some((ref remote_weather, remote_temp)) => is == remote_weather && temp.gt.as_str().parse::<f64>().unwrap() <= remote_temp,
                    }
            },
            Restriction::Or(or_restriction) => acc_predicate || or_restriction.check_request(arguments.clone(), weather_and_temp.clone()),
            Restriction::And(and_restriction) => acc_predicate && and_restriction.check_request(arguments.clone(), weather_and_temp.clone()),
        })
    }
}

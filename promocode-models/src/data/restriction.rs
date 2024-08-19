use chrono::{NaiveDate, Utc};
use log::error;
use serde::{Deserialize, Serialize};

use promocode::restriction_shadow_as_restriction;

use crate::data::_shadow::RestrictionShadow;
use crate::data::promocode;
use crate::data::temp::Temp;
use crate::extensions::vec_restriction::{Restrictions, RestrictionsExt};
use crate::req::promocode_request::Arguments;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "RestrictionShadow")]
pub enum Restriction {
    #[serde(rename = "@date")]
    Date { after: String, before: String },

    #[serde(rename = "@age")]
    Age {
        #[serde(skip_serializing_if = "Option::is_none")]
        lt: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        eq: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gt: Option<u8>,
    },

    #[serde(rename = "@meteo")]
    Meteo { is: String, temp: Temp },

    #[serde(rename = "@and")]
    And(Restrictions),

    #[serde(rename = "@or")]
    Or(Restrictions),
}

impl Restriction {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    /// Validate the Restriction.
    ///
    /// # Errors
    ///
    /// Returns an [Err] if the Restriction is not valid, with a String
    /// describing the error. Otherwise, returns [Ok] with a cloned copy of the
    /// [Restriction].
    pub fn validate(&self) -> Result<Self, /* Error */ String> {
        match &self {
            Restriction::Date { before, after } => {
                match (
                    NaiveDate::parse_from_str(before.as_str(), "%Y-%m-%d"),
                    NaiveDate::parse_from_str(after.as_str(), "%Y-%m-%d"),
                ) {
                    (Err(_), Err(_)) => Err("Cannot parse `before` and `after`.".to_string()),
                    (Err(_), Ok(_)) => Err("Cannot parse `before`.".to_string()),
                    (Ok(_), Err(_)) => Err("Cannot parse `after`.".to_string()),
                    #[allow(unused_variables)]
                    (Ok(before_date), Ok(after_date)) => {
                        if after_date <= before_date {
                            Ok(self.clone())
                        } else {
                            Err("`before` cannot be lower than `after`.".to_string())
                        }
                    },
                }
            },
            Restriction::Age { lt, eq, gt } => {
                if lt.is_none() && eq.is_none() && gt.is_none() {
                    return Err("One of `lt`, `eq` or `gt` must be present.".to_string());
                }
                match (gt, eq, lt) {
                    (None, Some(_), None) => Ok(self.clone()),
                    (Some(_), None, None) => Ok(self.clone()),
                    (None, None, Some(_)) => Ok(self.clone()),
                    (Some(gt_u8), None, Some(lt_u8)) => {
                        if gt_u8 < lt_u8 {
                            Ok(self.clone())
                        } else {
                            Err("`gt` cannot be lower than `lt`.".to_string())
                        }
                    },
                    (_, _, _) => Err("Unsupported @age restriction.".to_string()),
                }
            },
            Restriction::Meteo { is, temp } => {
                if is.is_empty() {
                    return Err("`is` must be nonempty.".to_string());
                }

                if let Err(err) = temp.validate() {
                    return Err(err.to_string());
                }

                Ok(self.clone())
            },
            Restriction::And(predicate) => {
                if predicate.is_empty() {
                    return Err("`@and` restrictions must be nonempty.".to_string());
                }
                let predicate_result_collected: Vec<Result<Restriction, /* Error */ String>> = predicate.iter().map(|it| it.validate()).collect();

                if let Some(err) = predicate_result_collected.iter().find(|it| it.is_err()) {
                    return Err(err.clone().unwrap_err());
                }

                Ok(self.clone())
            },
            Restriction::Or(predicate) => {
                if predicate.is_empty() {
                    return Err("`@or` restrictions must be nonempty.".to_string());
                }
                let predicate_result_collected: Vec<Result<Restriction, /* Error */ String>> = predicate.iter().map(|it| it.validate()).collect();

                if let Some(err) = predicate_result_collected.iter().find(|it| it.is_err()) {
                    return Err(err.clone().unwrap_err());
                }

                Ok(self.clone())
            },
        }
    }

    /// Collect a [Vec]<[Result]<[Restriction], [String]>> and return a [Result]
    /// of [Restrictions].
    ///
    /// # Arguments
    ///
    /// - `predicate_result_collected` - A vector of
    ///   [Result]<[Restriction], [String]>, representing the results of the
    ///   predicate.
    ///
    /// # Returns
    ///
    /// - Returns a [Result] containing either the collected [Restrictions] or
    ///   an error if any of the predicate results are [Err].
    fn collect_predicate(
        predicate_result_collected: Vec<Result<Restriction, String>>,
    ) -> Result<Restrictions, Result<Restriction, <Restriction as TryFrom<RestrictionShadow>>::Error>> {
        let predicate_collected = match predicate_result_collected.iter().find(|it| it.is_err()) {
            None => predicate_result_collected.iter().map(|it| it.clone().unwrap()).collect(),
            Some(err) => return Err(Err(err.clone().unwrap_err())),
        };
        Ok(predicate_collected)
    }

    /// Checks if the request satisfies one of the given [Restrictions]. Returns
    /// a boolean indicating whether the request is valid or not.
    ///
    /// (Implicit [crate::data::restriction::Restriction])
    ///
    /// # Arguments
    ///
    /// - `arguments` - Requested arguments.
    /// - `weather_and_temp` - The optional weather condition and temperature.
    pub fn check_restriction_generic(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool {
        match self {
            Restriction::Date { after, before } => Self::check_restriction_date(after, before),
            Restriction::Age { lt, eq, gt } => Self::check_restriction_age(&arguments, lt, eq, gt),
            Restriction::Meteo { is, temp } => Self::check_restriction_meteo(&weather_and_temp, is, temp),
            Restriction::Or(or_restriction) => or_restriction.check_restriction_or(arguments.clone(), weather_and_temp.clone()),
            Restriction::And(and_restriction) => and_restriction.check_restriction_and(arguments.clone(), weather_and_temp.clone()),
        }
    }

    /// Checks if the request satisfies [Restriction::Date]. Returns a boolean
    /// indicating whether the request is valid or not.
    ///
    /// # Arguments
    ///
    /// - `after` - Requested max date (default: [NaiveDate::MAX]).
    /// - `before` - Requested min date (default: [NaiveDate::MIN]).
    fn check_restriction_date(after: &String, before: &String) -> bool {
        let now = Utc::now().date_naive();
        let after_date = NaiveDate::parse_from_str(after.as_str(), "%Y-%m-%d").unwrap_or(NaiveDate::MIN);
        let before_date = NaiveDate::parse_from_str(before.as_str(), "%Y-%m-%d").unwrap_or(NaiveDate::MAX);

        after_date <= now && now <= before_date
    }

    /// Checks if the request satisfies [Restriction::Age]. Returns a boolean
    /// indicating whether the request is valid or not.
    ///
    /// One of `lt`, `eq` or `gt` must be different to [None]
    ///
    /// # Arguments
    ///
    /// - `arguments` - Requested arguments.
    /// - `lt` - Requested "lower than".
    /// - `eq` - Requested "equal".
    /// - `gt` - Requested "greater than".
    fn check_restriction_age(arguments: &Arguments, lt: &Option<u8>, eq: &Option<u8>, gt: &Option<u8>) -> bool {
        match (gt, eq, lt) {
            (None, Some(eq_u8), None) => &arguments.age == eq_u8,
            (Some(gt_u8), None, None) => &arguments.age >= gt_u8,
            (None, None, Some(lt_u8)) => &arguments.age <= lt_u8,
            (Some(gt_u8), None, Some(lt_u8)) => gt_u8 <= &arguments.age && &arguments.age <= lt_u8,
            _ => false,
        }
    }

    /// Checks if the request satisfies [Restriction::Meteo]. Returns a boolean
    /// indicating whether the request is valid or not.
    ///
    /// # Arguments
    ///
    /// - `weather_and_temp` - Current weather and temperature from remote.
    /// - `is` - Requested weather.
    /// - `temp` - Requested temperature.
    fn check_restriction_meteo(weather_and_temp: &Option<(String, f64)>, is: &String, temp: &Temp) -> bool {
        match weather_and_temp {
            None => {
                error!("Skip meteo check and return false because open_weather_sdk_unchecked is None.");
                false
            },
            Some((ref remote_weather, remote_temp)) => is == remote_weather && temp.gt.as_str().parse::<f64>().unwrap() <= *remote_temp,
        }
    }
}

impl TryFrom<RestrictionShadow> for Restriction {
    type Error = String;

    fn try_from(value: RestrictionShadow) -> Result<Self, Self::Error> {
        match value {
            RestrictionShadow::Date { before, after } => {
                match (
                    NaiveDate::parse_from_str(before.as_str(), "%Y-%m-%d"),
                    NaiveDate::parse_from_str(after.as_str(), "%Y-%m-%d"),
                ) {
                    (Err(_), Err(_)) => Err("Cannot parse `before` and `after`.".to_string()),
                    (Err(_), Ok(_)) => Err("Cannot parse `before`.".to_string()),
                    (Ok(_), Err(_)) => Err("Cannot parse `after`.".to_string()),
                    #[allow(unused_variables)]
                    (Ok(before_date), Ok(after_date)) => {
                        if after_date <= before_date {
                            Ok(Restriction::Date {
                                before: before.to_string(),
                                after: after.to_string(),
                            })
                        } else {
                            Err("`before` cannot be lower than `after`.".to_string())
                        }
                    },
                }
            },
            RestrictionShadow::Age { lt, eq, gt } => {
                if lt.is_none() && eq.is_none() && gt.is_none() {
                    return Err("One of `lt`, `eq` or `gt` must be present.".to_string());
                }
                match (gt, eq, lt) {
                    (None, Some(_), None) => Ok(Restriction::Age { lt, eq, gt }),
                    (Some(_), None, None) => Ok(Restriction::Age { lt, eq, gt }),
                    (None, None, Some(_)) => Ok(Restriction::Age { lt, eq, gt }),
                    (Some(gt_u8), None, Some(lt_u8)) => {
                        if gt_u8 < lt_u8 {
                            Ok(Restriction::Age { lt, eq, gt })
                        } else {
                            Err("`gt` cannot be lower than `lt`.".to_string())
                        }
                    },
                    (_, _, _) => Err("Unsupported @age restriction.".to_string()),
                }
            },
            RestrictionShadow::Meteo { is, temp } => {
                if is.is_empty() {
                    return Err("`is` must be nonempty.".to_string());
                }

                let temp_converted = match Temp::try_from(temp) {
                    Ok(result) => result,
                    Err(err) => return Err(err.to_string()),
                };

                Ok(Restriction::Meteo {
                    is: is.to_string(),
                    temp: temp_converted,
                })
            },
            RestrictionShadow::And(predicate) => {
                if predicate.is_empty() {
                    return Err("`@and` restrictions must be nonempty.".to_string());
                }
                match Self::collect_predicate(predicate.iter().map(restriction_shadow_as_restriction()).collect()) {
                    Ok(value) => Ok(Restriction::And(value)),
                    Err(value) => value,
                }
            },
            RestrictionShadow::Or(predicate) => {
                if predicate.is_empty() {
                    return Err("`@or` restrictions must be nonempty.".to_string());
                }
                match Self::collect_predicate(predicate.iter().map(restriction_shadow_as_restriction()).collect()) {
                    Ok(value) => Ok(Restriction::Or(value)),
                    Err(value) => value,
                }
            },
        }
    }
}

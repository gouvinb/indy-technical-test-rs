use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use promocode::restriction_shadow_as_restriction;

use crate::data::_shadow::RestrictionShadow;
use crate::data::promocode;
use crate::data::temp::Temp;
use crate::extensions::vec_restriction::Restrictions;

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
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use promocode_models::data::restriction::Restriction;
    ///
    /// let predicates : Vec<Result<Restriction, String>> = vec![];
    ///
    /// match Restriction::collect_predicate(predicates) {
    ///     Ok(value) => Ok(Restriction::And(value)),
    ///     Err(value) => value,
    /// }
    /// ```
    fn collect_predicate(
        predicate_result_collected: Vec<Result<Restriction, String>>,
    ) -> Result<Restrictions, Result<Restriction, <Restriction as TryFrom<RestrictionShadow>>::Error>> {
        let predicate_collected = match predicate_result_collected.iter().find(|it| it.is_err()) {
            None => predicate_result_collected.iter().map(|it| it.clone().unwrap()).collect(),
            Some(err) => return Err(Err(err.clone().unwrap_err())),
        };
        Ok(predicate_collected)
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

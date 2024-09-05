use crate::{
    promocode::{
        restrictions::{RestrictionsExt, SubRestrictions},
        temp::Temp,
    },
    promocode_request::arguments::Arguments,
};
use chrono::{NaiveDate, Utc};
use log::error;
use promocode_util::validate_type::{number::BoundedU8, string::NonBlankString};
use serde::{de::Error, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub enum Restriction {
    #[serde(rename = "@date")]
    Date {
        after: NonBlankString,
        before: NonBlankString,
    },

    #[serde(rename = "@age")]
    Age {
        #[serde(skip_serializing_if = "Option::is_none")]
        lt: Option<BoundedU8<0, { u8::MAX }>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        eq: Option<BoundedU8<0, { u8::MAX }>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gt: Option<BoundedU8<0, { u8::MAX }>>,
    },

    #[serde(rename = "@meteo")]
    Meteo { is: NonBlankString, temp: Temp },

    #[serde(rename = "@and")]
    And(SubRestrictions),

    #[serde(rename = "@or")]
    Or(SubRestrictions),
}

impl Restriction {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    /// Create a new [`Restriction::Date`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `Restriction::Date` is not correct.
    pub fn date(after: String, before: String) -> Result<Self, String> {
        let after = match NonBlankString::new(after) {
            Ok(value) => value,
            Err(err_after) => return Err(format!("`after` {}", err_after)),
        };
        let before = match NonBlankString::new(before) {
            Ok(value) => value,
            Err(err_before) => return Err(format!("`before` {}", err_before)),
        };

        match (
            NaiveDate::parse_from_str(before.clone().get().as_str(), "%Y-%m-%d"),
            NaiveDate::parse_from_str(after.clone().get().as_str(), "%Y-%m-%d"),
        ) {
            (Err(_), Err(_)) => Err("Cannot parse `before` and `after`.".to_string()),
            (Err(_), Ok(_)) => Err("Cannot parse `before`.".to_string()),
            (Ok(_), Err(_)) => Err("Cannot parse `after`.".to_string()),
            (Ok(before_date), Ok(after_date)) => {
                if after_date > before_date {
                    return Err("`before` cannot be lower than `after`.".to_string());
                }
                Ok(Restriction::Date { after, before })
            },
        }
    }

    /// Create a new [Restriction::Date] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it assumes that
    /// `Restriction::Date` is correct, without performing any validation.
    /// It's up to the caller to ensure any `after` and `before` string passed
    /// are correctly formatted dates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use promocode_models::promocode::restriction::Restriction;
    ///
    /// let restriction = unsafe {
    ///     Restriction::date_unchecked(
    ///         "2022-01-01".to_string(),
    ///         "2022-02-01".to_string(),
    ///     )
    /// };
    /// ```
    ///
    /// This function assumes `Restriction::Date` is correct.
    pub unsafe fn date_unchecked(after: String, before: String) -> Self {
        Self::Date {
            after: NonBlankString::new_unchecked(after),
            before: NonBlankString::new_unchecked(before),
        }
    }

    /// Create a new [`Restriction::Age`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `Restriction::Age` is not correct.
    pub fn age(lt: Option<u8>, eq: Option<u8>, gt: Option<u8>) -> Result<Self, String> {
        fn convert_to_bounded_u8(age: u8, field_name: String) -> Result<BoundedU8<0, { u8::MAX }>, String> {
            match BoundedU8::new(age) {
                Ok(value) => Ok(value),
                Err(err_after) => Err(format!("`{}` is out of bounds: {}", field_name, err_after)),
            }
        }
        fn convert_to_option_bounded_u8(age: Option<u8>, field_name: String) -> Result<Option<BoundedU8<0, { u8::MAX }>>, String> {
            match age {
                None => Ok(None),
                Some(age) => match convert_to_bounded_u8(age, field_name.clone()) {
                    Ok(value) => Ok(Some(value)),
                    Err(err) => Err(err),
                },
            }
        }

        let lt = match convert_to_option_bounded_u8(lt, "lt".to_string()) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };
        let eq = match convert_to_option_bounded_u8(eq, "eq".to_string()) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };
        let gt = match convert_to_option_bounded_u8(gt, "gt".to_string()) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

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
    }

    /// Create a new [Restriction::Age] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it creates a `Restriction::Age`
    /// without performing any validation on the inputs. It's up to the caller
    /// to ensure that the `lt`, `eq` and `gt` parameters passed to this
    /// function are valid for the age restriction being created.
    pub unsafe fn age_unchecked(lt: Option<u8>, eq: Option<u8>, gt: Option<u8>) -> Self {
        Self::Age {
            lt: lt.map(|value| BoundedU8::new_unchecked(value)),
            eq: eq.map(|value| BoundedU8::new_unchecked(value)),
            gt: gt.map(|value| BoundedU8::new_unchecked(value)),
        }
    }

    /// Create a new [`Restriction::Meteo`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `Restriction::Meteo` is not correct.
    pub fn meteo(is: String, temp: Temp) -> Result<Self, String> {
        let is = match NonBlankString::new(is) {
            Err(err_name) => return Err(format!("`is` {}", err_name)),
            Ok(value) => value,
        };

        Ok(Self::Meteo { is, temp })
    }

    /// Create a new [Restriction::Meteo] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it creates a
    /// `Restriction::Meteo` without performing any validation on the inputs.
    /// It's up to the caller to ensure that the `is` parameter passed to this
    /// function is a valid weather condition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use promocode_models::promocode::{
    ///     restriction::Restriction,
    ///     temp::Temp
    /// };
    ///
    /// let restriction = unsafe {
    ///     Restriction::meteo_unchecked(
    ///         "sunny".to_string(),
    ///         Temp { gt: 30 },
    ///     )
    /// };
    /// ```
    ///
    /// This function assumes `Restriction::Meteo` is correct.
    pub unsafe fn meteo_unchecked(is: String, temp: Temp) -> Self {
        Self::Meteo {
            is: NonBlankString::new_unchecked(is),
            temp,
        }
    }

    /// Create a new [`Restriction::And`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `Restriction::And` is not correct.
    pub fn and(restrictions: Vec<Result<Restriction, String>>) -> Result<Self, String> {
        match SubRestrictions::from_vec(restrictions) {
            Err(err) => Err(err),
            Ok(value) => Ok(Self::And(value)),
        }
    }

    /// Create a new [Restriction::And] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it creates a
    /// `Restriction::And` without performing any validation on the inputs.
    ///
    /// This function assumes `Restriction::And` is correct.
    pub unsafe fn and_unchecked(restrictions: SubRestrictions) -> Self {
        Self::And(restrictions)
    }

    /// Create a new [`Restriction::Or`](Self)
    ///
    /// # Errors
    ///
    /// This function fails if `Restriction::Or` is not correct.
    pub fn or(restrictions: Vec<Result<Restriction, String>>) -> Result<Self, String> {
        match SubRestrictions::from_vec(restrictions) {
            Err(err) => Err(err),
            Ok(value) => Ok(Self::Or(value)),
        }
    }

    /// Create a new [Restriction::Or] (unchecked)
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it creates a
    /// `Restriction::Or` without performing any validation on the inputs.
    ///
    /// This function assumes `Restriction::Or` is correct.
    pub unsafe fn or_unchecked(restrictions: SubRestrictions) -> Self {
        Self::Or(restrictions)
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
    fn check_restriction_date(after: &NonBlankString, before: &NonBlankString) -> bool {
        let now = Utc::now().date_naive();
        let after_date = NaiveDate::parse_from_str(after.clone().get().as_str(), "%Y-%m-%d").unwrap_or(NaiveDate::MIN);
        let before_date = NaiveDate::parse_from_str(before.clone().get().as_str(), "%Y-%m-%d").unwrap_or(NaiveDate::MAX);

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
    fn check_restriction_age(
        arguments: &Arguments,
        lt: &Option<BoundedU8<0, { u8::MAX }>>,
        eq: &Option<BoundedU8<0, { u8::MAX }>>,
        gt: &Option<BoundedU8<0, { u8::MAX }>>,
    ) -> bool {
        match (gt, eq, lt) {
            (None, Some(eq_u8), None) => arguments.age == eq_u8.get(),
            (Some(gt_u8), None, None) => arguments.age >= gt_u8.get(),
            (None, None, Some(lt_u8)) => arguments.age <= lt_u8.get(),
            (Some(gt_u8), None, Some(lt_u8)) => gt_u8.get() <= arguments.age && arguments.age <= lt_u8.get(),
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
    fn check_restriction_meteo(weather_and_temp: &Option<(String, f64)>, is: &NonBlankString, temp: &Temp) -> bool {
        match weather_and_temp {
            None => {
                error!("Skip meteo check and return false because open_weather_sdk_unchecked is None.");
                false
            },
            Some((ref remote_weather, remote_temp)) => &is.clone().get() == remote_weather && temp.gt as f64 <= *remote_temp,
        }
    }
}

impl<'de> Deserialize<'de> for Restriction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map: HashMap<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;

        if let Some(value) = map.remove("@date") {
            #[derive(Deserialize)]
            struct DateUnsafe {
                after: String,
                before: String,
            }
            let date: DateUnsafe = serde_json::from_value(value).map_err(D::Error::custom)?;
            return match Restriction::date(date.after, date.before) {
                Ok(result) => Ok(result),
                Err(err) => Err(Error::custom(err)),
            };
        }

        if let Some(value) = map.remove("@age") {
            #[derive(Deserialize)]
            struct AgeUnsafe {
                #[serde(skip_serializing_if = "Option::is_none")]
                lt: Option<u8>,
                #[serde(skip_serializing_if = "Option::is_none")]
                eq: Option<u8>,
                #[serde(skip_serializing_if = "Option::is_none")]
                gt: Option<u8>,
            }
            let age: AgeUnsafe = serde_json::from_value(value).map_err(D::Error::custom)?;
            return match Restriction::age(age.lt, age.eq, age.gt) {
                Ok(result) => Ok(result),
                Err(err) => Err(Error::custom(err)),
            };
        }

        if let Some(value) = map.remove("@meteo") {
            #[derive(Deserialize)]
            struct MeteoUnsafe {
                is: String,
                temp: Temp,
            }
            let meteo: MeteoUnsafe = serde_json::from_value(value).map_err(D::Error::custom)?;
            return match Restriction::meteo(meteo.is, meteo.temp) {
                Ok(result) => Ok(result),
                Err(err) => Err(Error::custom(err)),
            };
        }

        if let Some(value) = map.remove("@and") {
            let and: SubRestrictions = serde_json::from_value(value).map_err(D::Error::custom)?;
            return Ok(Restriction::And(and));
        }

        if let Some(value) = map.remove("@or") {
            let or: SubRestrictions = serde_json::from_value(value).map_err(D::Error::custom)?;
            return Ok(Restriction::Or(or));
        }

        Err(D::Error::custom("Unknown restriction type"))
    }
}

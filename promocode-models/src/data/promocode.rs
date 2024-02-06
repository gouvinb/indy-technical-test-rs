use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::data::promocode_shadow::{AvantageShadow, PromocodeShadow, RestrictionShadow, TempShadow};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "PromocodeShadow")]
pub struct Promocode {
    pub _id: String,
    pub name: String,
    pub avantage: Avantage,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<Restriction>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "AvantageShadow")]
pub struct Avantage {
    pub percent: u8,
}

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
    And(Vec<Restriction>),

    #[serde(rename = "@or")]
    Or(Vec<Restriction>),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "TempShadow")]
pub struct Temp {
    pub gt: String,
}

impl Promocode {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    pub fn validate(&self) -> Result<Promocode, /* Error */ String> {
        if self._id.is_empty() {
            return Err("`_id` must be nonempty.".to_string());
        }

        if self.name.is_empty() {
            return Err("`name` must be nonempty.".to_string());
        }

        self.avantage.validate()?;

        // WARN: must be uncommented if restrictions can be empty
        // if self.restrictions.is_empty() {
        //     return Err("`restrictions` must be nonempty.".to_string());
        // }

        let restrictions_result_collected: Vec<Result<Restriction, String>> = self.restrictions.iter().map(|it| it.validate()).collect();

        if let Some(err) = restrictions_result_collected.iter().find(|it| it.is_err()) {
            return Err(err.clone().unwrap_err());
        }

        Ok(self.clone())
    }
}

impl Avantage {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    pub fn validate(&self) -> Result<Avantage, /* Error */ String> {
        // TODO: Ask to product: percent may be 0 ?
        if !(1u8..=100u8).contains(&self.percent) {
            return Err("`percent` must be greater than 0 and lower than 101.".to_string());
        }

        Ok(self.clone())
    }
}

impl Restriction {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    fn validate(&self) -> Result<Self, /* Error */ String> {
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
                    (Ok(before_date), Ok(after_date)) => Ok(self.clone()),
                }
            },
            Restriction::Age { lt, eq, gt } => {
                if lt.is_none() && eq.is_none() && gt.is_none() {
                    return Err("One of `lt`, `eq` or `gt` must be present.".to_string());
                }
                Ok(self.clone())
            },
            #[allow(unused_variables)]
            Restriction::Meteo { is, temp } => {
                // WARN: must be uncommented if Meteo have `is` validation
                // if is.is_empty() {
                //     Err("`is` must be nonempty.")
                // }

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
}

impl Temp {
    // type Error = String; // <- Case error[E0658]: inherent associated types are unstable

    fn validate(&self) -> Result<Self, /* Error */ String> {
        // WARN: must be uncommented if Temp have `gt` emtpy check
        // if value.gt.is_empty() {
        //     return Err("`gt` must be nonempty.".to_string());
        // }

        if self.gt.as_str().parse::<i16>().is_err() {
            return Err("`gt` must be an int.".to_string());
        }

        Ok(self.clone())
    }
}

impl TryFrom<PromocodeShadow> for Promocode {
    type Error = String;

    fn try_from(value: PromocodeShadow) -> Result<Self, Self::Error> {
        if value._id.is_empty() {
            return Err("`_id` must be nonempty.".to_string());
        }

        if value.name.is_empty() {
            return Err("`name` must be nonempty.".to_string());
        }

        let avantage_converted = match Avantage::try_from(value.avantage) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        // WARN: must be uncommented if restrictions can be empty
        // if value.restrictions.is_empty() {
        //     return Err("`restrictions` must be nonempty.".to_string());
        // }

        let restrictions_result_collected: Vec<Result<Restriction, Self::Error>> = value.restrictions.iter().map(restriction_shadow_as_restriction()).collect();

        let restrictions_collected = match restrictions_result_collected.iter().find(|it| it.is_err()) {
            None => restrictions_result_collected.iter().map(|it| it.clone().unwrap()).collect(),
            Some(err) => return Err(err.clone().unwrap_err()),
        };

        Ok(Promocode {
            _id: value._id,
            name: value.name,
            avantage: avantage_converted,
            restrictions: restrictions_collected,
        })
    }
}

impl TryFrom<AvantageShadow> for Avantage {
    type Error = String;

    fn try_from(value: AvantageShadow) -> Result<Self, Self::Error> {
        // TODO: Ask to product: percent may be 0 ?
        if !(1u8..=100u8).contains(&value.percent) {
            return Err("`percent` must be greater than 0 and lower than 101.".to_string());
        }

        Ok(Avantage { percent: value.percent })
    }
}

impl TryFrom<RestrictionShadow> for Restriction {
    type Error = String;

    fn try_from(value: RestrictionShadow) -> Result<Self, Self::Error> {
        match value {
            // TODO: Ask to product: it's a closed range ?
            RestrictionShadow::Date { before, after } => {
                match (
                    NaiveDate::parse_from_str(before.as_str(), "%Y-%m-%d"),
                    NaiveDate::parse_from_str(after.as_str(), "%Y-%m-%d"),
                ) {
                    (Err(_), Err(_)) => Err("Cannot parse `before` and `after`.".to_string()),
                    (Err(_), Ok(_)) => Err("Cannot parse `before`.".to_string()),
                    (Ok(_), Err(_)) => Err("Cannot parse `after`.".to_string()),
                    #[allow(unused_variables)]
                    (Ok(before_date), Ok(after_date)) => Ok(Restriction::Date {
                        before: before.to_string(),
                        after: after.to_string(),
                    }),
                }
            },
            RestrictionShadow::Age { lt, eq, gt } => {
                if lt.is_none() && eq.is_none() && gt.is_none() {
                    return Err("One of `lt`, `eq` or `gt` must be present.".to_string());
                }
                Ok(Restriction::Age { lt, eq, gt })
            },
            RestrictionShadow::Meteo { is, temp } => {
                // WARN: must be uncommented if Meteo have `is` validation
                // if is.is_empty() {
                //     Err("`is` must be nonempty.")
                // }

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
                let predicate_result_collected: Vec<Result<Restriction, Self::Error>> = predicate.iter().map(restriction_shadow_as_restriction()).collect();

                let predicate_collected = match predicate_result_collected.iter().find(|it| it.is_err()) {
                    None => predicate_result_collected.iter().map(|it| it.clone().unwrap()).collect(),
                    Some(err) => return Err(err.clone().unwrap_err()),
                };

                Ok(Restriction::And(predicate_collected))
            },
            RestrictionShadow::Or(predicate) => {
                if predicate.is_empty() {
                    return Err("`@or` restrictions must be nonempty.".to_string());
                }
                let predicate_result_collected: Vec<Result<Restriction, Self::Error>> = predicate.iter().map(restriction_shadow_as_restriction()).collect();

                let predicate_collected = match predicate_result_collected.iter().find(|it| it.is_err()) {
                    None => predicate_result_collected.iter().map(|it| it.clone().unwrap()).collect(),
                    Some(err) => return Err(err.clone().unwrap_err()),
                };

                Ok(Restriction::Or(predicate_collected))
            },
        }
    }
}

impl TryFrom<TempShadow> for Temp {
    type Error = String;

    fn try_from(value: TempShadow) -> Result<Self, Self::Error> {
        // WARN: must be uncommented if Temp have `gt` emtpy check
        // if value.gt.is_empty() {
        //     return Err("`gt` must be nonempty.".to_string());
        // }
        if value.gt.as_str().parse::<i16>().is_err() {
            return Err("`gt` must be an int.".to_string());
        }

        Ok(Temp { gt: value.gt.to_string() })
    }
}

fn restriction_shadow_as_restriction() -> fn(&RestrictionShadow) -> Result<Restriction, /* Error */ String> {
    |it| Restriction::try_from(it.clone())
}

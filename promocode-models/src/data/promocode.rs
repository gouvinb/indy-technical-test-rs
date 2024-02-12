use serde::{Deserialize, Serialize};

use crate::data::_shadow::{PromocodeShadow, RestrictionShadow};
use crate::data::avantage::Avantage;
use crate::data::restriction::Restriction;
use crate::extensions::vec_restriction::Restrictions;
use crate::res::promocode_accepted;
use crate::res::promocode_accepted::PromocodeAccepted;
use crate::res::promocode_denied::{PromocodeDenied, Reasons};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "PromocodeShadow")]
pub struct Promocode {
    pub _id: String,
    pub name: String,
    pub avantage: Avantage,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Restrictions,
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

        if self.restrictions.len() > 2 {
            return Err("`restrictions` must contain 0, 1 or 2 entries.".to_string());
        }

        if !self.restrictions.is_empty() {
            if let Restriction::And(_) | Restriction::Or(_) = self.restrictions.first().unwrap() {
                return Err("The first restriction must a @date, @age or @meteo.".to_string());
            }

            if let (2, Restriction::And(_) | Restriction::Or(_)) = (self.restrictions.len(), self.restrictions.first().unwrap()) {
                return Err("The next restriction must a @and or @or.".to_string());
            }
        }

        let restrictions_result_collected: Vec<Result<Restriction, String>> = self.restrictions.iter().map(|it| it.validate()).collect();

        if let Some(err) = restrictions_result_collected.iter().find(|it| it.is_err()) {
            return Err(err.clone().unwrap_err());
        }

        Ok(self.clone())
    }

    pub fn generate_response(promocode_name: String, percent: u8, predicate: bool) -> Result<PromocodeAccepted, PromocodeDenied> {
        if predicate {
            Ok(PromocodeAccepted {
                promocode_name,
                status: "accepted".to_string(),
                avantage: promocode_accepted::Avantage { percent },
            })
        } else {
            Err(PromocodeDenied {
                promocode_name,
                status: "denied".to_string(),
                reasons: Reasons {},
            })
        }
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

        if let RestrictionShadow::And(_) | RestrictionShadow::Or(_) = value.restrictions.first().unwrap() {
            return Err("The first restriction must a @date, @age or @meteo.".to_string());
        }

        if let (2, RestrictionShadow::And(_) | RestrictionShadow::Or(_)) = (value.restrictions.len(), value.restrictions.first().unwrap()) {
            return Err("The next restriction must a @and or @or.".to_string());
        }

        // WARN: restrictions must have @date, @age or @meteo AND POTENTIALLY @and ot @or

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

pub(crate) fn restriction_shadow_as_restriction() -> fn(&RestrictionShadow) -> Result<Restriction, /* Error */ String> {
    |it| Restriction::try_from(it.clone())
}

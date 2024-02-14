use chrono::{NaiveDate, Utc};
use log::{debug, error, info};

use crate::data::restriction::Restriction;
use crate::req::promocode_request::Arguments;

pub type Restrictions = Vec<Restriction>;

pub trait RestrictionsExt {
    fn check_request(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool;
}

impl RestrictionsExt for Restrictions {
    fn check_request(&self, arguments: Arguments, weather_and_temp: Option<(String, f64)>) -> bool {
        let first_restriction_check = match self.first().unwrap() {
            Restriction::Date { after, before } => {
                let now = Utc::now().date_naive();
                let after_date = NaiveDate::parse_from_str(after.as_str(), "%Y-%m-%d").unwrap_or(NaiveDate::MIN);
                let before_date = NaiveDate::parse_from_str(before.as_str(), "%Y-%m-%d").unwrap_or(NaiveDate::MAX);

                info!("after_date = {}, now = {}, before_date = {}", after_date, now, before_date);

                after_date <= now && now <= before_date
            },
            Restriction::Age { lt, eq, gt } => match (gt, eq, lt) {
                (None, Some(eq_u8), None) => &arguments.age == eq_u8,
                (Some(gt_u8), None, None) => &arguments.age >= gt_u8,
                (None, None, Some(lt_u8)) => &arguments.age <= lt_u8,
                (Some(gt_u8), None, Some(lt_u8)) => gt_u8 <= &arguments.age && &arguments.age <= lt_u8,
                _ => false,
            },
            Restriction::Meteo { is, temp } => match weather_and_temp {
                None => {
                    error!("Skip meteo check and return false because open_weather_sdk_unchecked is None.");
                    false
                },
                Some((ref remote_weather, remote_temp)) => {
                    debug!("{is} == {remote_weather} && {} < {remote_temp}", temp.gt.as_str().parse::<f64>().unwrap());
                    is == remote_weather && temp.gt.as_str().parse::<f64>().unwrap() <= remote_temp
                },
            },
            _ => {
                return false;
            },
        };

        match self.get(1) {
            Some(Restriction::Or(or_restriction)) => first_restriction_check || or_restriction.check_request(arguments, weather_and_temp.clone()),
            Some(Restriction::And(and_restriction)) => first_restriction_check && and_restriction.check_request(arguments, weather_and_temp.clone()),
            None => first_restriction_check,
            _ => false,
        }
    }
}

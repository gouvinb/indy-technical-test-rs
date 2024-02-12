use crate::data::restriction::Restriction;
use crate::req::promocode_request::Arguments;
use chrono::{NaiveDate, Utc};
use log::info;

pub type Restrictions = Vec<Restriction>;

pub trait RestrictionsExt {
    fn check_request(&self, arguments: Arguments, open_weather_map_api_key: String) -> bool;
}

impl RestrictionsExt for Restrictions {
    fn check_request(&self, arguments: Arguments, open_weather_map_api_key: String) -> bool {
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
            Restriction::Meteo { .. } => {
                if open_weather_map_api_key.is_empty() {
                    false
                } else {
                    // TODO: Use Open Weather API
                    true
                }
            },
            _ => {
                return false;
            },
        };

        match self.get(1) {
            Some(Restriction::Or(or_restriction)) => first_restriction_check || or_restriction.check_request(arguments, open_weather_map_api_key),
            Some(Restriction::And(and_restriction)) => first_restriction_check && and_restriction.check_request(arguments, open_weather_map_api_key),
            None => first_restriction_check,
            _ => false,
        }
    }
}

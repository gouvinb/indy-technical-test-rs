use chrono::{Datelike, TimeDelta, Utc};
use promocode_models::{
    promocode::{avantage::Avantage, restriction::Restriction, restrictions::RestrictionsExt, temp::Temp, Promocode},
    promocode_request::{arguments::Arguments, meteo::Meteo, PromocodeRequest},
};

#[test]
fn check_request_date() {
    let now_date_naive = Utc::now().date_naive();
    let date_fmt_str = "%Y-%m-%d";

    let promocode_with_past_date = Promocode::new(
        "id - past date".to_string(),
        "past date".to_string(),
        Avantage::new(10),
        vec![Restriction::date(
            (now_date_naive - TimeDelta::days(365))
                .format(date_fmt_str)
                .to_string(),
            (now_date_naive - TimeDelta::days(365))
                .format(date_fmt_str)
                .to_string(),
        )],
    )
    .unwrap();

    let promocode_with_future_date = Promocode::new(
        "id - future date".to_string(),
        "future date".to_string(),
        Avantage::new(10),
        vec![Restriction::date(
            (now_date_naive + TimeDelta::days(365))
                .format(date_fmt_str)
                .to_string(),
            (now_date_naive + TimeDelta::days(365))
                .format(date_fmt_str)
                .to_string(),
        )],
    )
    .unwrap();

    let promocode_with_in_range_date = Promocode::new(
        "id - in range".to_string(),
        "in range".to_string(),
        Avantage::new(10),
        vec![Restriction::date(
            (now_date_naive - TimeDelta::days(30))
                .format(date_fmt_str)
                .to_string(),
            (now_date_naive + TimeDelta::days(30))
                .format(date_fmt_str)
                .to_string(),
        )],
    )
    .unwrap();

    let promocode_with_useless_case_date = Promocode::new(
        "id - today".to_string(),
        "today".to_string(),
        Avantage::new(10),
        vec![
            Restriction::date(
                (now_date_naive - TimeDelta::days(1))
                    .format(date_fmt_str)
                    .to_string(),
                (now_date_naive + TimeDelta::days(1))
                    .format(date_fmt_str)
                    .to_string(),
            ),
            Restriction::date(
                now_date_naive
                    .with_day(now_date_naive.day())
                    .unwrap()
                    .format(date_fmt_str)
                    .to_string(),
                now_date_naive
                    .with_day(now_date_naive.day())
                    .unwrap()
                    .format(date_fmt_str)
                    .to_string(),
            ),
        ],
    )
    .unwrap();

    let promocode_with_today_date = Promocode::new(
        "id - today".to_string(),
        "today".to_string(),
        Avantage::new(10),
        vec![Restriction::date(
            now_date_naive.format(date_fmt_str).to_string(),
            now_date_naive.format(date_fmt_str).to_string(),
        )],
    )
    .unwrap();

    let request = PromocodeRequest::new(
        "future date".to_string(),
        Arguments::new(25, Meteo::new("Lyon".to_string())),
    )
    .unwrap();

    assert_eq!(
        promocode_with_past_date
            .restrictions
            .check_restriction_or(request.arguments.clone(), None),
        false
    );
    assert_eq!(
        promocode_with_future_date
            .restrictions
            .check_restriction_or(request.arguments.clone(), None),
        false
    );
    assert_eq!(
        promocode_with_in_range_date
            .restrictions
            .check_restriction_or(request.arguments.clone(), None),
        true
    );
    assert_eq!(
        promocode_with_useless_case_date
            .restrictions
            .check_restriction_or(request.arguments.clone(), None),
        true
    );
    assert_eq!(
        promocode_with_today_date
            .restrictions
            .check_restriction_or(request.arguments.clone(), None),
        true
    );
}

#[test]
fn check_request_age() {
    let promocode_with_eq_30_age = Promocode::new(
        "id - age testing - eq 30".to_string(),
        "age testing - eq 30".to_string(),
        Avantage::new(10),
        vec![Restriction::age(None, Some(30), None)],
    )
    .unwrap();

    let promocode_with_lt_30_age = Promocode::new(
        "id - age testing - lt 30".to_string(),
        "age testing - lt 30".to_string(),
        Avantage::new(10),
        vec![Restriction::age(Some(30), None, None)],
    )
    .unwrap();

    let promocode_with_gt_30_age = Promocode::new(
        "id - age testing - gt 30".to_string(),
        "age testing - gt 30".to_string(),
        Avantage::new(10),
        vec![Restriction::age(None, None, Some(30))],
    )
    .unwrap();

    let promocode_with_range_20_40_age = Promocode::new(
        "id - age testing - range 20..40".to_string(),
        "age testing - range 20..40".to_string(),
        Avantage::new(10),
        vec![Restriction::age(Some(40), None, Some(20))],
    )
    .unwrap();

    let request_base = |promocode_name: String, age: u8| {
        PromocodeRequest::new(
            promocode_name,
            Arguments::new(age, Meteo::new("Lyon".to_string())),
        )
        .unwrap()
    };

    assert_eq!(
        promocode_with_eq_30_age.restrictions.check_restriction_or(
            request_base("age testing - eq 30".to_string(), 31).arguments,
            None
        ),
        false
    );
    assert_eq!(
        promocode_with_eq_30_age.restrictions.check_restriction_or(
            request_base("age testing - eq 30".to_string(), 30).arguments,
            None
        ),
        true
    );
    assert_eq!(
        promocode_with_eq_30_age.restrictions.check_restriction_or(
            request_base("age testing - eq 30".to_string(), 29).arguments,
            None
        ),
        false
    );

    assert_eq!(
        promocode_with_lt_30_age.restrictions.check_restriction_or(
            request_base("age testing - lt 30".to_string(), 31).arguments,
            None
        ),
        false
    );
    assert_eq!(
        promocode_with_lt_30_age.restrictions.check_restriction_or(
            request_base("age testing - lt 30".to_string(), 30).arguments,
            None
        ),
        true
    );
    assert_eq!(
        promocode_with_lt_30_age.restrictions.check_restriction_or(
            request_base("age testing - lt 30".to_string(), 29).arguments,
            None
        ),
        true
    );

    assert_eq!(
        promocode_with_gt_30_age.restrictions.check_restriction_or(
            request_base("age testing - gt 30".to_string(), 31).arguments,
            None
        ),
        true
    );
    assert_eq!(
        promocode_with_gt_30_age.restrictions.check_restriction_or(
            request_base("age testing - gt 30".to_string(), 30).arguments,
            None
        ),
        true
    );
    assert_eq!(
        promocode_with_gt_30_age.restrictions.check_restriction_or(
            request_base("age testing - gt 30".to_string(), 29).arguments,
            None
        ),
        false
    );

    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_restriction_or(
                request_base("age testing - range 20..40".to_string(), 19).arguments,
                None
            ),
        false
    );
    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_restriction_or(
                request_base("age testing - range 20..40".to_string(), 20).arguments,
                None
            ),
        true
    );
    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_restriction_or(
                request_base("age testing - range 20..40".to_string(), 30).arguments,
                None
            ),
        true
    );
    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_restriction_or(
                request_base("age testing - range 20..40".to_string(), 40).arguments,
                None
            ),
        true
    );
    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_restriction_or(
                request_base("age testing - range 20..40".to_string(), 41).arguments,
                None
            ),
        false
    );
}

#[test]
fn check_request_meteo() {
    let promocode_with_clear_15_meteo = Promocode::new(
        "id - meteo testing - clear 15".to_string(),
        "meteo testing - clear 15".to_string(),
        Avantage::new(10),
        vec![Restriction::meteo("clear".to_string(), Temp { gt: 15 })],
    )
    .unwrap();

    let request = PromocodeRequest::new(
        "meteo testing - clear 15".to_string(),
        Arguments::new(1, Meteo::new("Lyon".to_string())),
    )
    .unwrap();

    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(request.arguments.clone(), None),
        false
    );
    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(
                request.arguments.clone(),
                Some(("not clear".to_string(), 1f64))
            ),
        false
    );
    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(
                request.arguments.clone(),
                Some(("not clear".to_string(), 15f64))
            ),
        false
    );
    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(
                request.arguments.clone(),
                Some(("not clear".to_string(), 42f64))
            ),
        false
    );

    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(request.arguments.clone(), Some(("Clear".to_string(), 1f64))),
        false
    );
    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(
                request.arguments.clone(),
                Some(("Clear".to_string(), 15f64))
            ),
        false
    );
    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(
                request.arguments.clone(),
                Some(("Clear".to_string(), 42f64))
            ),
        false
    );

    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(request.arguments.clone(), Some(("clear".to_string(), 1f64))),
        false
    );
    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(
                request.arguments.clone(),
                Some(("clear".to_string(), 15f64))
            ),
        true
    );
    assert_eq!(
        promocode_with_clear_15_meteo
            .restrictions
            .check_restriction_or(
                request.arguments.clone(),
                Some(("clear".to_string(), 42f64))
            ),
        true
    );
}

#[test]
fn check_request_and_or() {
    let promocode_with_eq_19_age_or_20_40_age = Promocode::new(
        "id - and/or testing - eq 19 or 20..40".to_string(),
        "and/or testing - eq 19 or 20..40".to_string(),
        Avantage::new(10),
        vec![
            Restriction::age(None, Some(19), None),
            Restriction::and(vec![
                Restriction::age(None, None, Some(20)),
                Restriction::age(Some(40), None, None),
            ]),
        ],
    )
    .unwrap();

    let request_with_18_age = PromocodeRequest::new(
        "and/or testing - eq 19 or 20..40".to_string(),
        Arguments::new(18, Meteo::new("Lyon".to_string())),
    )
    .unwrap();

    let request_with_41_age = PromocodeRequest::new(
        "and/or testing - eq 19 or 20..40".to_string(),
        Arguments::new(41, Meteo::new("Lyon".to_string())),
    )
    .unwrap();

    let request_with_19_age = PromocodeRequest::new(
        "and/or testing - eq 19 or 20..40".to_string(),
        Arguments::new(19, Meteo::new("Lyon".to_string())),
    )
    .unwrap();

    let request_with_30_age = PromocodeRequest::new(
        "and/or testing - eq 19 or 20..40".to_string(),
        Arguments::new(30, Meteo::new("Lyon".to_string())),
    )
    .unwrap();

    assert_eq!(
        promocode_with_eq_19_age_or_20_40_age
            .restrictions
            .check_restriction_or(request_with_18_age.arguments, None),
        false
    );
    assert_eq!(
        promocode_with_eq_19_age_or_20_40_age
            .restrictions
            .check_restriction_or(request_with_41_age.arguments, None),
        false
    );
    assert_eq!(
        promocode_with_eq_19_age_or_20_40_age
            .restrictions
            .check_restriction_or(request_with_19_age.arguments, None),
        true
    );
    assert_eq!(
        promocode_with_eq_19_age_or_20_40_age
            .restrictions
            .check_restriction_or(request_with_30_age.arguments, None),
        true
    );
}

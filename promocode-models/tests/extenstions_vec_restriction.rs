use chrono::{Datelike, Utc};

use promocode_models::data::avantage::Avantage;
use promocode_models::data::promocode::Promocode;
use promocode_models::data::restriction::Restriction;
use promocode_models::data::temp::Temp;
use promocode_models::extensions::vec_restriction::RestrictionsExt;
use promocode_models::req::promocode_request::{Arguments, Meteo, PromocodeRequest};

#[test]
fn check_request_date() {
    let now_date_naive = Utc::now().date_naive();
    let date_fmt_str = "%Y-%m-%d";

    let promocode_with_past_date = Promocode {
        _id: "id - past date".to_string(),
        name: "past date".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![Restriction::Date {
            after: now_date_naive.with_year(now_date_naive.year() - 1).unwrap().format(date_fmt_str).to_string(),
            before: now_date_naive.with_year(now_date_naive.year() - 1).unwrap().format(date_fmt_str).to_string(),
        }],
    };

    let promocode_with_future_date = Promocode {
        _id: "id - future date".to_string(),
        name: "future date".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![Restriction::Date {
            after: now_date_naive.with_year(now_date_naive.year() + 1).unwrap().format(date_fmt_str).to_string(),
            before: now_date_naive.with_year(now_date_naive.year() + 1).unwrap().format(date_fmt_str).to_string(),
        }],
    };

    let promocode_with_in_range_date = Promocode {
        _id: "id - in range".to_string(),
        name: "in range".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![Restriction::Date {
            after: now_date_naive.with_month(now_date_naive.month() - 1).unwrap().format(date_fmt_str).to_string(),
            before: now_date_naive.with_month(now_date_naive.month() + 1).unwrap().format(date_fmt_str).to_string(),
        }],
    };

    let promocode_with_today_date = Promocode {
        _id: "id - today".to_string(),
        name: "today".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![Restriction::Date {
            after: now_date_naive.format(date_fmt_str).to_string(),
            before: now_date_naive.format(date_fmt_str).to_string(),
        }],
    };

    let request = PromocodeRequest {
        promocode_name: "future date".to_string(),
        arguments: Arguments {
            age: 25,
            meteo: Meteo { town: "Lyon".to_string() },
        },
    };

    assert_eq!(
        promocode_with_past_date.restrictions.check_request(request.arguments.clone(), "".to_string(),),
        false
    );
    assert_eq!(
        promocode_with_future_date
            .restrictions
            .check_request(request.arguments.clone(), "".to_string(),),
        false
    );
    assert_eq!(
        promocode_with_in_range_date
            .restrictions
            .check_request(request.arguments.clone(), "".to_string(),),
        true
    );
    assert_eq!(
        promocode_with_today_date.restrictions.check_request(request.arguments.clone(), "".to_string(),),
        true
    );
}

#[test]
fn check_request_age() {
    let promocode_with_eq_30_age = Promocode {
        _id: "id - age testing - eq 30".to_string(),
        name: "age testing - eq 30".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![Restriction::Age {
            lt: None,
            eq: Some(30),
            gt: None,
        }],
    };

    let promocode_with_lt_30_age = Promocode {
        _id: "id - age testing - lt 30".to_string(),
        name: "age testing - lt 30".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![Restriction::Age {
            lt: Some(30),
            eq: None,
            gt: None,
        }],
    };

    let promocode_with_gt_30_age = Promocode {
        _id: "id - age testing - gt 30".to_string(),
        name: "age testing - gt 30".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![Restriction::Age {
            lt: None,
            eq: None,
            gt: Some(30),
        }],
    };

    let promocode_with_range_20_40_age = Promocode {
        _id: "id - age testing - range 20..40".to_string(),
        name: "age testing - range 20..40".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![Restriction::Age {
            lt: Some(40),
            eq: None,
            gt: Some(20),
        }],
    };

    let request_base = |promocode_name: String, age: u8| PromocodeRequest {
        promocode_name: promocode_name,
        arguments: Arguments {
            age: age,
            meteo: Meteo { town: "Lyon".to_string() },
        },
    };

    assert_eq!(
        promocode_with_eq_30_age
            .restrictions
            .check_request(request_base("age testing - eq 30".to_string(), 31).arguments, "".to_string()),
        false
    );
    assert_eq!(
        promocode_with_eq_30_age
            .restrictions
            .check_request(request_base("age testing - eq 30".to_string(), 30).arguments, "".to_string()),
        true
    );
    assert_eq!(
        promocode_with_eq_30_age
            .restrictions
            .check_request(request_base("age testing - eq 30".to_string(), 29).arguments, "".to_string()),
        false
    );

    assert_eq!(
        promocode_with_lt_30_age
            .restrictions
            .check_request(request_base("age testing - lt 30".to_string(), 31).arguments, "".to_string()),
        false
    );
    assert_eq!(
        promocode_with_lt_30_age
            .restrictions
            .check_request(request_base("age testing - lt 30".to_string(), 30).arguments, "".to_string()),
        true
    );
    assert_eq!(
        promocode_with_lt_30_age
            .restrictions
            .check_request(request_base("age testing - lt 30".to_string(), 29).arguments, "".to_string()),
        true
    );

    assert_eq!(
        promocode_with_gt_30_age
            .restrictions
            .check_request(request_base("age testing - gt 30".to_string(), 31).arguments, "".to_string()),
        true
    );
    assert_eq!(
        promocode_with_gt_30_age
            .restrictions
            .check_request(request_base("age testing - gt 30".to_string(), 30).arguments, "".to_string()),
        true
    );
    assert_eq!(
        promocode_with_gt_30_age
            .restrictions
            .check_request(request_base("age testing - gt 30".to_string(), 29).arguments, "".to_string()),
        false
    );

    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_request(request_base("age testing - range 20..40".to_string(), 19).arguments, "".to_string()),
        false
    );
    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_request(request_base("age testing - range 20..40".to_string(), 20).arguments, "".to_string()),
        true
    );
    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_request(request_base("age testing - range 20..40".to_string(), 30).arguments, "".to_string()),
        true
    );
    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_request(request_base("age testing - range 20..40".to_string(), 40).arguments, "".to_string()),
        true
    );
    assert_eq!(
        promocode_with_range_20_40_age
            .restrictions
            .check_request(request_base("age testing - range 20..40".to_string(), 41).arguments, "".to_string()),
        false
    );
}

#[test]
fn check_request_meteo() {
    let promocode_with_clear_15_meteo = Promocode {
        _id: "id - meteo testing - clear 15".to_string(),
        name: "meteo testing - clear 15".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![
            Restriction::Meteo {
                is: "clear".to_string(),
                temp: Temp { gt: "15".to_string() },
            },
        ],
    };

    let request = PromocodeRequest {
        promocode_name: "meteo testing - clear 15".to_string(),
        arguments: Arguments {
            age: 1,
            meteo: Meteo { town: "Lyon".to_string() },
        },
    };

    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), None), false);
    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("not clear".to_string(), 1f64))), false);
    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("not clear".to_string(), 15f64))), false);
    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("not clear".to_string(), 42f64))), false);

    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("Clear".to_string(), 1f64))), false);
    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("Clear".to_string(), 15f64))), false);
    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("Clear".to_string(), 42f64))), false);

    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("clear".to_string(), 1f64))), false);
    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("clear".to_string(), 15f64))), true);
    assert_eq!(promocode_with_clear_15_meteo.restrictions.check_request(request.arguments.clone(), Some(("clear".to_string(), 42f64))), true);
}

#[test]
fn check_request_and_or() {
    let promocode_with_eq_19_age_or_20_40_age = Promocode {
        _id: "id - and/or testing - eq 19 or 20..40".to_string(),
        name: "and/or testing - eq 19 or 20..40".to_string(),
        avantage: Avantage { percent: 10 },
        restrictions: vec![
            Restriction::Age {
                lt: Some(40),
                eq: None,
                gt: None,
            },
            Restriction::And(vec![
                Restriction::Age {
                    lt: None,
                    eq: None,
                    gt: Some(20),
                },
                Restriction::Or(vec![Restriction::Age {
                    lt: None,
                    eq: Some(19),
                    gt: None,
                }]),
            ]),
        ],
    };

    let request_with_18_age = PromocodeRequest {
        promocode_name: "and/or testing - eq 19 or 20..40".to_string(),
        arguments: Arguments {
            age: 18,
            meteo: Meteo { town: "Lyon".to_string() },
        },
    };

    let request_with_41_age = PromocodeRequest {
        promocode_name: "and/or testing - eq 19 or 20..40".to_string(),
        arguments: Arguments {
            age: 41,
            meteo: Meteo { town: "Lyon".to_string() },
        },
    };

    let request_with_19_age = PromocodeRequest {
        promocode_name: "and/or testing - eq 19 or 20..40".to_string(),
        arguments: Arguments {
            age: 19,
            meteo: Meteo { town: "Lyon".to_string() },
        },
    };

    let request_with_30_age = PromocodeRequest {
        promocode_name: "and/or testing - eq 19 or 20..40".to_string(),
        arguments: Arguments {
            age: 30,
            meteo: Meteo { town: "Lyon".to_string() },
        },
    };

    assert_eq!(promocode_with_eq_19_age_or_20_40_age.restrictions.check_request(request_with_18_age.arguments, "".to_string()), false);
    assert_eq!(promocode_with_eq_19_age_or_20_40_age.restrictions.check_request(request_with_41_age.arguments, "".to_string()), false);
    assert_eq!(promocode_with_eq_19_age_or_20_40_age.restrictions.check_request(request_with_19_age.arguments, "".to_string()), true);
    assert_eq!(promocode_with_eq_19_age_or_20_40_age.restrictions.check_request(request_with_30_age.arguments, "".to_string()), true);
}

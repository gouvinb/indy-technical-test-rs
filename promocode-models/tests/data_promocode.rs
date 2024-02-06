use promocode_models::data::promocode::Restriction::{Age, And, Date, Meteo, Or};
use promocode_models::data::promocode::{Avantage, Promocode, Temp};

#[test]
fn promocode_validation() {
    let promocode_valid = Promocode {
        _id: "...".to_string(),
        name: "WeatherCode".to_string(),
        avantage: Avantage { percent: 20 },
        restrictions: vec![
            Date {
                after: "2019-01-01".to_string(),
                before: "2020-06-30".to_string(),
            },
            Or(vec![
                Age {
                    lt: None,
                    eq: Some(40),
                    gt: None,
                },
                And(vec![
                    Age {
                        lt: Some(30),
                        eq: None,
                        gt: Some(15),
                    },
                    Meteo {
                        is: "clear".to_string(),
                        temp: Temp { gt: "15".to_string() },
                    },
                ]),
            ]),
        ],
    }
    .validate();

    assert!(promocode_valid.is_ok());

    let promocode_with_empty_id = Promocode {
        _id: "".to_string(),
        name: "WeatherCode".to_string(),
        avantage: Avantage { percent: 20 },
        restrictions: vec![
            Date {
                after: "2019-01-01".to_string(),
                before: "2020-06-30".to_string(),
            },
            Or(vec![
                Age {
                    lt: None,
                    eq: Some(40),
                    gt: None,
                },
                And(vec![
                    Age {
                        lt: Some(30),
                        eq: None,
                        gt: Some(15),
                    },
                    Meteo {
                        is: "clear".to_string(),
                        temp: Temp { gt: "15".to_string() },
                    },
                ]),
            ]),
        ],
    }
    .validate();

    assert!(promocode_with_empty_id.is_err());

    let promocode_with_empty_name = Promocode {
        _id: "id".to_string(),
        name: "".to_string(),
        avantage: Avantage { percent: 20 },
        restrictions: vec![
            Date {
                after: "2019-01-01".to_string(),
                before: "2020-06-30".to_string(),
            },
            Or(vec![
                Age {
                    lt: None,
                    eq: Some(40),
                    gt: None,
                },
                And(vec![
                    Age {
                        lt: Some(30),
                        eq: None,
                        gt: Some(15),
                    },
                    Meteo {
                        is: "clear".to_string(),
                        temp: Temp { gt: "15".to_string() },
                    },
                ]),
            ]),
        ],
    }
    .validate();

    assert!(promocode_with_empty_name.is_err());

    let promocode_with_0_percent_to_avantage = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 0 },
        restrictions: vec![
            Date {
                after: "2019-01-01".to_string(),
                before: "2020-06-30".to_string(),
            },
            Or(vec![
                Age {
                    lt: None,
                    eq: Some(40),
                    gt: None,
                },
                And(vec![
                    Age {
                        lt: Some(30),
                        eq: None,
                        gt: Some(15),
                    },
                    Meteo {
                        is: "clear".to_string(),
                        temp: Temp { gt: "15".to_string() },
                    },
                ]),
            ]),
        ],
    }
    .validate();

    assert!(promocode_with_0_percent_to_avantage.is_err());

    let promocode_with_101_percent_to_avantage = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 101 },
        restrictions: vec![
            Date {
                after: "2019-01-01".to_string(),
                before: "2020-06-30".to_string(),
            },
            Or(vec![
                Age {
                    lt: None,
                    eq: Some(40),
                    gt: None,
                },
                And(vec![
                    Age {
                        lt: Some(30),
                        eq: None,
                        gt: Some(15),
                    },
                    Meteo {
                        is: "clear".to_string(),
                        temp: Temp { gt: "15".to_string() },
                    },
                ]),
            ]),
        ],
    }
    .validate();

    assert!(promocode_with_101_percent_to_avantage.is_err());

    let promocode_without_restriction = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 42 },
        restrictions: vec![],
    }
    .validate();

    assert!(promocode_without_restriction.is_ok());

    let promocode_with_invalid_date_before_restriction = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 42 },
        restrictions: vec![Date {
            before: "2024-13-32".to_string(),
            after: "2020-06-30".to_string(),
        }],
    }
    .validate();

    assert!(promocode_with_invalid_date_before_restriction.is_err());

    let promocode_with_invalid_date_after_restriction = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 42 },
        restrictions: vec![Date {
            after: "2024-13-32".to_string(),
            before: "2020-06-30".to_string(),
        }],
    }
    .validate();

    assert!(promocode_with_invalid_date_after_restriction.is_err());

    let promocode_with_invalid_date_restriction = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 42 },
        restrictions: vec![Date {
            after: "2024-13-32".to_string(),
            before: "2024-13-32".to_string(),
        }],
    }
    .validate();

    assert!(promocode_with_invalid_date_restriction.is_err());

    let promocode_with_invalid_age_restriction = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 42 },
        restrictions: vec![Age { lt: None, eq: None, gt: None }],
    }
    .validate();

    assert!(promocode_with_invalid_age_restriction.is_err());

    // WARN: must be uncommented if Meteo have `is` validation
    // let promocode_with_invalid_meteo_is_restriction = Promocode {
    //     _id: "id".to_string(),
    //     name: "name".to_string(),
    //     avantage: Avantage { percent: 42 },
    //     restrictions: vec![
    //         Meteo { is: "".to_string(), temp: Temp { gt: "0".to_string() } },
    //     ],
    // }.validate();
    //
    // assert!(promocode_with_invalid_meteo_is_restriction.is_err());

    // WARN: must be uncommented if Temp have `gt` emtpy check
    // let promocode_with_empty_meteo_temp_gt_restriction = Promocode {
    //     _id: "id".to_string(),
    //     name: "name".to_string(),
    //     avantage: Avantage { percent: 42 },
    //     restrictions: vec![
    //         Meteo { is: "is".to_string(), temp: Temp { gt: "".to_string() } },
    //     ],
    // }.validate();
    //
    // assert!(promocode_with_empty_meteo_temp_gt_restriction.is_err());

    let promocode_with_invalid_meteo_temp_gt_restriction = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 42 },
        restrictions: vec![Meteo {
            is: "is".to_string(),
            temp: Temp { gt: "not a i16".to_string() },
        }],
    }
    .validate();

    assert!(promocode_with_invalid_meteo_temp_gt_restriction.is_err());

    let promocode_with_empty_and_restriction = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 42 },
        restrictions: vec![And(vec![])],
    }
    .validate();

    assert!(promocode_with_empty_and_restriction.is_err());

    let promocode_with_empty_or_restriction = Promocode {
        _id: "id".to_string(),
        name: "name".to_string(),
        avantage: Avantage { percent: 42 },
        restrictions: vec![Or(vec![])],
    }
    .validate();

    assert!(promocode_with_empty_or_restriction.is_err());
}

#[test]
fn promocode_ser_de() {
    let promocode_valid = Promocode {
        _id: "...".to_string(),
        name: "WeatherCode".to_string(),
        avantage: Avantage { percent: 20 },
        restrictions: vec![
            Date {
                after: "2019-01-01".to_string(),
                before: "2020-06-30".to_string(),
            },
            Or(vec![
                Age {
                    lt: None,
                    eq: Some(40),
                    gt: None,
                },
                And(vec![
                    Age {
                        lt: Some(30),
                        eq: None,
                        gt: Some(15),
                    },
                    Meteo {
                        is: "clear".to_string(),
                        temp: Temp { gt: "15".to_string() },
                    },
                ]),
            ]),
        ],
    }
    .validate();

    assert!(promocode_valid.is_ok());
    let promocode = promocode_valid.unwrap();
    let serialized_result = serde_json::to_string(&promocode);

    assert!(serialized_result.is_ok());
    let serialized = serialized_result.unwrap();

    let promocode_str = r#"{"_id":"...","name":"WeatherCode","avantage":{"percent":20},"restrictions":[{"@date":{"after":"2019-01-01","before":"2020-06-30"}},{"@or":[{"@age":{"eq":40}},{"@and":[{"@age":{"lt":30,"gt":15}},{"@meteo":{"is":"clear","temp":{"gt":"15"}}}]}]}]}"#;

    let deserialized_result = serde_json::from_str::<Promocode>(&promocode_str);

    assert!(deserialized_result.is_ok());
    let deserialized = deserialized_result.unwrap();

    assert_eq!(promocode, deserialized);
    assert_eq!(serialized, promocode_str);
}

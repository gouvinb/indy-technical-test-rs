use promocode_models::res::promocode_accepted::{Avantage, PromocodeAccepted};

#[test]
fn promocode_accepted_validation() {
    let promocode_accepted_valid = PromocodeAccepted {
        promocode_name: "promocode_name".to_string(),
        status: "status".to_string(),
        avantage: Avantage { percent: 42 },
    }
    .validate();

    assert!(promocode_accepted_valid.is_ok());

    let promocode_accepted_with_empty_promocode_name = PromocodeAccepted {
        promocode_name: "".to_string(),
        status: "status".to_string(),
        avantage: Avantage { percent: 42 },
    }
    .validate();

    assert!(promocode_accepted_with_empty_promocode_name.is_err());

    let promocode_accepted_with_bad_avantage = PromocodeAccepted {
        promocode_name: "promocode_name".to_string(),
        status: "status".to_string(),
        avantage: Avantage { percent: 0 },
    }
    .validate();

    assert!(promocode_accepted_with_bad_avantage.is_err());
}

#[test]
fn promocode_accepted_serde() {
    let promocode_accepted_valid = PromocodeAccepted {
        promocode_name: "WeatherCode".to_string(),
        status: "accepted".to_string(),
        avantage: Avantage { percent: 20 },
    }
    .validate();

    assert!(promocode_accepted_valid.is_ok());
    let promocode_accepted = promocode_accepted_valid.unwrap();
    let serialized_result = serde_json::to_string(&promocode_accepted);

    assert!(serialized_result.is_ok());
    let serialized = serialized_result.unwrap();

    let promocode_str = r#"{"promocode_name":"WeatherCode","status":"accepted","avantage":{"percent":20}}"#;

    let deserialized_result = serde_json::from_str::<PromocodeAccepted>(&promocode_str);

    assert!(deserialized_result.is_ok());
    let deserialized = deserialized_result.unwrap();

    assert_eq!(promocode_accepted, deserialized);
    assert_eq!(serialized, promocode_str);
}

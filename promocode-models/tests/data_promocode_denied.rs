use promocode_models::res::promocode_denied::{PromocodeDenied, Reasons};

#[test]
fn promocode_denied_validation() {
    let promocode_denied_valid = PromocodeDenied {
        promocode_name: "promocode_name".to_string(),
        status: "status".to_string(),
        reasons: Reasons {},
    }
        .validate();

    assert!(promocode_denied_valid.is_ok());

    let promocode_denied_with_empty_promocode_name = PromocodeDenied {
        promocode_name: "".to_string(),
        status: "status".to_string(),
        reasons: Reasons {},
    }
        .validate();

    assert!(promocode_denied_with_empty_promocode_name.is_err());
}

#[test]
fn promocode_denied_serde() {
    let promocode_denied_valid = PromocodeDenied {
        promocode_name: "WeatherCode".to_string(),
        status: "denied".to_string(),
        reasons: Reasons {},
    }
        .validate();

    assert!(promocode_denied_valid.is_ok());
    let promocode_denied = promocode_denied_valid.unwrap();
    let serialized_result = serde_json::to_string(&promocode_denied);

    assert!(serialized_result.is_ok());
    let serialized = serialized_result.unwrap();

    let promocode_str = r#"{"promocode_name":"WeatherCode","status":"denied","reasons":{}}"#;

    let deserialized_result = serde_json::from_str::<PromocodeDenied>(&promocode_str);

    assert!(deserialized_result.is_ok());
    let deserialized = deserialized_result.unwrap();

    assert_eq!(promocode_denied, deserialized);
    assert_eq!(serialized, promocode_str);
}


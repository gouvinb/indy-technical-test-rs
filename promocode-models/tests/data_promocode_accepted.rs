use promocode_models::{promocode::avantage::Avantage, promocode_response::PromocodeResponse};

#[test]
fn promocode_accepted_validation() {
    let promocode_accepted_valid = PromocodeResponse::accepted("promocode_name".to_string(), Avantage::new(42).unwrap());

    assert!(promocode_accepted_valid.is_ok());

    let promocode_accepted_with_empty_promocode_name = PromocodeResponse::accepted("".to_string(), Avantage::new(42).unwrap());

    assert!(promocode_accepted_with_empty_promocode_name.is_err());
}

#[test]
fn promocode_accepted_serde() {
    let promocode_accepted_valid = PromocodeResponse::accepted("WeatherCode".to_string(), Avantage::new(20).unwrap());

    assert!(promocode_accepted_valid.is_ok());
    let promocode_accepted = promocode_accepted_valid.unwrap();
    let serialized_result = serde_json::to_string(&promocode_accepted);

    assert!(serialized_result.is_ok());
    let serialized = serialized_result.unwrap();

    let promocode_str = r#"{"promocode_name":"WeatherCode","status":"accepted","avantage":{"percent":20}}"#;

    let deserialized_result = serde_json::from_str::<PromocodeResponse>(&promocode_str);

    assert!(deserialized_result.is_ok());
    let deserialized = deserialized_result.unwrap();

    assert_eq!(promocode_accepted, deserialized);
    assert_eq!(serialized, promocode_str);
}

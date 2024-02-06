use promocode_models::req::promocode_request::{Arguments, Meteo, PromocodeRequest};

#[test]
fn promocode_request_validation() {
    let promocode_request_valid = PromocodeRequest {
        promocode_name: "promocode_name".to_string(),
        arguments: Arguments {
            age: 42,
            meteo: Meteo { town: "town".to_string() },
        },
    }
    .validate();

    assert!(promocode_request_valid.is_ok());

    let promocode_request_with_empty_promocode_name = PromocodeRequest {
        promocode_name: "".to_string(),
        arguments: Arguments {
            age: 42,
            meteo: Meteo { town: "town".to_string() },
        },
    }
    .validate();

    assert!(promocode_request_with_empty_promocode_name.is_err());

    let promocode_request_with_empty_meteo_town = PromocodeRequest {
        promocode_name: "promocode_name".to_string(),
        arguments: Arguments {
            age: 42,
            meteo: Meteo { town: "".to_string() },
        },
    }
    .validate();

    assert!(promocode_request_with_empty_meteo_town.is_err());
}

#[test]
fn promocode_request_serde() {
    let promocode_request_valid = PromocodeRequest {
        promocode_name: "WeatherCode".to_string(),
        arguments: Arguments {
            age: 25,
            meteo: Meteo { town: "Lyon".to_string() },
        },
    }
    .validate();

    assert!(promocode_request_valid.is_ok());
    let promocode_request = promocode_request_valid.unwrap();
    let serialized_result = serde_json::to_string(&promocode_request);

    assert!(serialized_result.is_ok());
    let serialized = serialized_result.unwrap();

    let promocode_str = r#"{"promocode_name":"WeatherCode","arguments":{"age":25,"meteo":{"town":"Lyon"}}}"#;

    let deserialized_result = serde_json::from_str::<PromocodeRequest>(&promocode_str);

    assert!(deserialized_result.is_ok());
    let deserialized = deserialized_result.unwrap();

    assert_eq!(promocode_request, deserialized);
    assert_eq!(serialized, promocode_str);
}

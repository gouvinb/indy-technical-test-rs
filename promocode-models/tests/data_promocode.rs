use promocode_models::promocode::{
    avantage::Avantage,
    restriction::Restriction::{self, And, Or},
    temp::Temp,
    Promocode,
};
use promocode_util::validate_type::sequence::NonEmptyVec;

#[test]
fn promocode_validation() {
    let promocode_complex_valid = Promocode::new(
        "...".to_string(),
        "WeatherCode".to_string(),
        Avantage::new(20).unwrap(),
        vec![
            Restriction::date("2019-01-01".to_string(), "2020-06-30".to_string()).unwrap(),
            Or(NonEmptyVec::new(vec![
                Restriction::age(None, Some(40), None).unwrap(),
                And(NonEmptyVec::new(vec![
                    Restriction::age(Some(30), None, Some(15)).unwrap(),
                    Restriction::meteo("clear".to_string(), Temp { gt: 15 }).unwrap(),
                ])
                .unwrap()),
            ])
            .unwrap()),
        ],
    );

    assert!(promocode_complex_valid.is_ok());

    let promocode_with_empty_id = Promocode::new(
        "".to_string(),
        "WeatherCode".to_string(),
        Avantage::new(20).unwrap(),
        vec![
            Restriction::date("2019-01-01".to_string(), "2020-06-30".to_string()).unwrap(),
            Or(NonEmptyVec::new(vec![
                Restriction::age(None, Some(40), None).unwrap(),
                And(NonEmptyVec::new(vec![
                    Restriction::age(Some(30), None, Some(15)).unwrap(),
                    Restriction::meteo("clear".to_string(), Temp { gt: 15 }).unwrap(),
                ])
                .unwrap()),
            ])
            .unwrap()),
        ],
    );

    assert!(promocode_with_empty_id.is_err());

    let promocode_with_empty_name = Promocode::new(
        "id".to_string(),
        "".to_string(),
        Avantage::new(20).unwrap(),
        vec![
            Restriction::date("2019-01-01".to_string(), "2020-06-30".to_string()).unwrap(),
            Or(NonEmptyVec::new(vec![
                Restriction::age(None, Some(40), None).unwrap(),
                And(NonEmptyVec::new(vec![
                    Restriction::age(Some(30), None, Some(15)).unwrap(),
                    Restriction::meteo("clear".to_string(), Temp { gt: 15 }).unwrap(),
                ])
                .unwrap()),
            ])
            .unwrap()),
        ],
    );

    assert!(promocode_with_empty_name.is_err());

    let promocode_with_0_percent_to_avantage = Promocode::new(
        "id".to_string(),
        "name".to_string(),
        Avantage::new(0).unwrap(),
        vec![
            Restriction::date("2019-01-01".to_string(), "2020-06-30".to_string()).unwrap(),
            Or(NonEmptyVec::new(vec![
                Restriction::age(None, Some(40), None).unwrap(),
                And(NonEmptyVec::new(vec![
                    Restriction::age(Some(30), None, Some(15)).unwrap(),
                    Restriction::meteo("clear".to_string(), Temp { gt: 15 }).unwrap(),
                ])
                .unwrap()),
            ])
            .unwrap()),
        ],
    );

    assert!(promocode_with_0_percent_to_avantage.is_ok());

    let promocode_without_restriction = Promocode::new(
        "id".to_string(),
        "name".to_string(),
        Avantage::new(42).unwrap(),
        vec![],
    );

    assert!(promocode_without_restriction.is_ok());

    let promocode_implicit_or_restriction = Promocode::new(
        "...".to_string(),
        "WeatherCode".to_string(),
        Avantage::new(20).unwrap(),
        vec![
            Restriction::date("2019-01-01".to_string(), "2020-06-30".to_string()).unwrap(),
            Restriction::age(None, Some(40), None).unwrap(),
            And(NonEmptyVec::new(vec![
                Restriction::age(Some(30), None, Some(15)).unwrap(),
                Restriction::meteo("clear".to_string(), Temp { gt: 15 }).unwrap(),
            ])
            .unwrap()),
        ],
    );

    assert!(promocode_implicit_or_restriction.is_ok());
}

#[test]
fn promocode_ser_de() {
    let promocode_valid = Promocode::new(
        "...".to_string(),
        "WeatherCode".to_string(),
        Avantage::new(20).unwrap(),
        vec![
            Restriction::date("2019-01-01".to_string(), "2020-06-30".to_string()).unwrap(),
            Or(NonEmptyVec::new(vec![
                Restriction::age(None, Some(40), None).unwrap(),
                And(NonEmptyVec::new(vec![
                    Restriction::age(Some(30), None, Some(15)).unwrap(),
                    Restriction::meteo("clear".to_string(), Temp { gt: 15 }).unwrap(),
                ])
                .unwrap()),
            ])
            .unwrap()),
        ],
    );

    assert!(promocode_valid.is_ok());
    let promocode = promocode_valid.unwrap();
    let serialized_result = serde_json::to_string(&promocode);

    assert!(serialized_result.is_ok());
    let serialized = serialized_result.unwrap();

    let promocode_str = "{\"_id\":\"...\",\"name\":\"WeatherCode\",\"avantage\":{\"percent\":20},\"restrictions\":[{\"@date\":{\"after\":\"2019-01-01\",\"before\":\"2020-06-30\"}},{\"@or\":[{\"@age\":{\"eq\":40}},{\"@and\":[{\"@age\":{\"lt\":30,\"gt\":15}},{\"@meteo\":{\"is\":\"clear\",\"temp\":{\"gt\":15}}}]}]}]}";

    let deserialized_result = serde_json::from_str::<Promocode>(&promocode_str);

    assert!(deserialized_result.is_ok());
    let deserialized = deserialized_result.unwrap();

    assert_eq!(promocode, deserialized);
    assert_eq!(serialized, promocode_str);
}

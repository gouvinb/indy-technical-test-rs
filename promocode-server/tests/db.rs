use promocode_models::data::avantage::Avantage;
use promocode_models::data::promocode::Promocode;
use promocode_server::db::{db_delete_by_id, db_delete_by_name, db_get_by_id, db_get_by_name, db_list, db_push};

#[test]
fn db_use_case() {
    assert!(db_list().is_empty());

    assert!(db_get_by_id("id".to_string()).is_none());
    assert!(db_get_by_name("name".to_string()).is_none());

    let promocode_invalid = Promocode {
        _id: "".to_string(),
        name: "no id set".to_string(),
        avantage: Avantage { percent: 1 },
        restrictions: vec![],
    };

    let promocode_0 = Promocode {
        _id: "0".to_string(),
        name: "name 0".to_string(),
        avantage: Avantage { percent: 1 },
        restrictions: vec![],
    };

    let promocode_1 = Promocode {
        _id: "1".to_string(),
        name: "name 1".to_string(),
        avantage: Avantage { percent: 1 },
        restrictions: vec![],
    };

    assert!(db_push(promocode_invalid.clone()).is_err());

    assert!(db_push(promocode_0.clone()).is_ok());
    assert_eq!(db_get_by_id("0".to_string()), Some(promocode_0.clone()));

    assert!(db_push(promocode_1.clone()).is_ok());
    assert_eq!(db_get_by_name("name 1".to_string()), Some(promocode_1.clone()));

    assert_eq!(db_list().to_vec(), vec![promocode_0.clone(), promocode_1.clone()]);

    db_delete_by_id("0".to_string());
    assert!(db_get_by_id("0".to_string()).is_none());
    assert_eq!(db_list().to_vec(), vec![promocode_1.clone()]);

    db_delete_by_name("name 1".to_string());
    assert!(db_get_by_name("name 1".to_string()).is_none());
    assert!(db_list().is_empty());
}

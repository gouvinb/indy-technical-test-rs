// WARN: DB for dev only !

use std::sync::{OnceLock, RwLock, RwLockReadGuard};

use promocode_models::data::promocode::Promocode;

fn db() -> &'static RwLock<Vec<Promocode>> {
    static DB: OnceLock<RwLock<Vec<Promocode>>> = OnceLock::new();
    DB.get_or_init(|| RwLock::new(vec![]))
}

pub fn db_list() -> RwLockReadGuard<'static, Vec<Promocode>> {
    db().read().unwrap()
}

pub fn db_get_by_id(id: String) -> Option<Promocode> {
    match db_list().iter().find(|promocode| promocode._id == id) {
        None => None,
        Some(value) => Some(value.clone()),
    }
}

pub fn db_get_by_name(name: String) -> Option<Promocode> {
    match db_list().iter().find(|promocode| promocode.name == name) {
        None => None,
        Some(value) => Some(value.clone()),
    }
}

pub fn db_push(promocode: Promocode) -> Result<(), /*Error*/ String> {
    match promocode.validate() {
        Ok(value) => Ok(db().write().unwrap().push(value)),
        Err(err) => Err(err),
    }
}

pub fn db_delete_by_id(id: String) {
    db().write().unwrap().retain(|promocode| promocode._id != id)
}

pub fn db_delete_by_name(name: String) {
    db().write().unwrap().retain(|promocode| promocode.name != name)
}

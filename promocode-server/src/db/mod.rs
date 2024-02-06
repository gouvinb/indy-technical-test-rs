// WARN: DB for dev only !

use std::sync::{OnceLock, RwLock};

use promocode_models::data::promocode::Promocode;

fn db() -> &'static RwLock<Vec<Promocode>> {
    static DB: OnceLock<RwLock<Vec<Promocode>>> = OnceLock::new();
    DB.get_or_init(|| RwLock::new(vec![]))
}

pub fn db_list() -> Vec<Promocode> {
    db().read().unwrap().to_vec()
}

pub fn db_get_by_id(id: String) -> Option<Promocode> {
    db_list().iter().find(|promocode| promocode._id == id).cloned()
}

pub fn db_get_by_name(name: String) -> Option<Promocode> {
    db_list().iter().find(|promocode| promocode.name == name).cloned()
}

pub fn db_push(promocode: Promocode) -> Result<(), /*Error*/ String> {
    if db_list().iter().any(|it| it._id == promocode._id || it.name == promocode.name) {
        return Err(format!("Promocode with id `{}` or name `{}` already exist.", promocode._id, promocode.name));
    }

    match promocode.validate() {
        Ok(value) => {
            db().write().unwrap().push(value);
            Ok(())
        },
        Err(err) => Err(err),
    }
}

pub fn db_delete_by_id(id: String) {
    db().write().unwrap().retain(|promocode| promocode._id != id)
}

pub fn db_delete_by_name(name: String) {
    db().write().unwrap().retain(|promocode| promocode.name != name)
}

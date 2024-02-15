//! # WARN: DB for dev only !

use std::sync::{OnceLock, RwLock};

use promocode_models::data::promocode::Promocode;

fn db() -> &'static RwLock<Vec<Promocode>> {
    static DB: OnceLock<RwLock<Vec<Promocode>>> = OnceLock::new();
    DB.get_or_init(|| RwLock::new(vec![]))
}

/// Retrieves a list of promocodes from the database.
pub fn db_list() -> Vec<Promocode> {
    db().read().unwrap().to_vec()
}

/// Retrieves a `Promocode` from the database by its ID.
pub fn db_get_by_id(id: String) -> Option<Promocode> {
    db_list().iter().find(|promocode| promocode._id == id).cloned()
}

/// Retrieves a `Promocode` from the database by name.
pub fn db_get_by_name(name: String) -> Option<Promocode> {
    db_list().iter().find(|promocode| promocode.name == name).cloned()
}

/// Pushes a new `Promocode` to the database.
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

/// Deletes the promocode from the database with the given ID.
pub fn db_delete_by_id(id: String) {
    db().write().unwrap().retain(|promocode| promocode._id != id)
}

/// Deletes all entries from the database with a given name.
pub fn db_delete_by_name(name: String) {
    db().write().unwrap().retain(|promocode| promocode.name != name)
}

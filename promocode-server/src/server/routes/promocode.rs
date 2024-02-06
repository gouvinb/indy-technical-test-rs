use actix_web::{delete, get, put, web, Error, HttpResponse};

use promocode_models::data::promocode::Promocode;

use crate::db::{db_delete_by_name, db_list, db_push};

pub fn promocode_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_promocode);
    cfg.service(put_promocode);
    if cfg!(debug_assertions) {
        cfg.service(delete_promocode);
    }
}

#[get("/promocode")]
pub async fn get_promocode() -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(""))
}

#[put("/promocode")]
pub async fn put_promocode(promocode_json: web::Json<Promocode>) -> HttpResponse {
    if db_list().iter().any(|it| it._id == promocode_json._id || it.name == promocode_json.name) {
        return HttpResponse::BadRequest().json(format!(
            "Promocode with id `{}` or name `{}` already exist.",
            promocode_json._id, promocode_json.name
        ));
    }

    match db_push(promocode_json.to_owned()) {
        Ok(_) => HttpResponse::Ok().json(""),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[delete("/promocode")]
pub async fn delete_promocode(name: web::Json<String>) -> HttpResponse {
    db_delete_by_name(name.to_owned());
    HttpResponse::Ok().json("")
}

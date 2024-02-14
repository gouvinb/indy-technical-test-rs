use actix_web::{delete, get, put, web, HttpResponse};

use promocode_models::data::promocode::Promocode;
use promocode_models::extensions::vec_restriction::RestrictionsExt;
use promocode_models::req::promocode_request::PromocodeRequest;

use crate::db::{db_delete_by_name, db_get_by_name, db_list, db_push};
use crate::open_weather_sdk;

pub fn promocode_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_promocode);
    cfg.service(put_promocode);
    if cfg!(debug_assertions) {
        cfg.service(delete_promocode);
    }
}

#[get("/promocode")]
pub async fn get_promocode(promocode_req_json: web::Json<PromocodeRequest>) -> HttpResponse {
    let mut percent = 0u8;

    let predicate = match db_get_by_name(promocode_req_json.promocode_name.clone()) {
        Some(promocode) => {
            percent = promocode.avantage.percent;

            let weather_and_temp = open_weather_sdk::get_current_meteo_and_temp(&promocode_req_json).await;

            promocode.restrictions.check_request(promocode_req_json.arguments.clone(), weather_and_temp)
        },
        None => false,
    };

    match Promocode::generate_response(promocode_req_json.promocode_name.clone(), percent, predicate) {
        Ok(promocode_accepted) => HttpResponse::Ok().json(promocode_accepted),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
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

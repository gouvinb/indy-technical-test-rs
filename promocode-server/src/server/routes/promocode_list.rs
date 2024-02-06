use actix_web::{get, web, HttpResponse};

use crate::db::db_list;

pub fn promocode_list_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_promocode_list);
}

#[get("/promocodes")]
async fn get_promocode_list() -> HttpResponse {
    HttpResponse::Ok().json(db_list())
}

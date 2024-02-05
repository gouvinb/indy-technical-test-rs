use actix_web::{get, web, Error, HttpResponse};

pub fn promocode_list_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_promocode_list);
}

#[get("/promocodes")]
async fn get_promocode_list() -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(""))
}

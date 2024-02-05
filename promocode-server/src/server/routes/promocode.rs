use actix_web::{delete, get, put, web, Error, HttpResponse};

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
pub async fn put_promocode() -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(""))
}

#[delete("/promocode")]
pub async fn delete_promocode() -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(""))
}

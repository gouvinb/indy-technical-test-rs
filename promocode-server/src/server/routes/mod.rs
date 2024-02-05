use actix_web::web;

pub mod promocode;
pub mod promocode_list;

pub fn services(cfg: &mut web::ServiceConfig) {
    promocode::promocode_services(cfg);

    if cfg!(debug_assertions) {
        promocode_list::promocode_list_services(cfg);
    }
}

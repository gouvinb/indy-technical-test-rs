use ntex::web::{get, HttpResponse, ServiceConfig};

use crate::db::db_list;

/// Register the `get_promocode_list` service to the given `ServiceConfig`.
///
/// # Arguments
///
/// - `cfg` - A mutable reference to the `ServiceConfig` to register the service
///   with.
///
pub fn promocode_list_services(cfg: &mut ServiceConfig) {
    cfg.service(get_promocode_list);
}

/// Fetches the list of [Promocode].
#[get("/promocodes")]
async fn get_promocode_list() -> HttpResponse {
    HttpResponse::Ok().json(&db_list())
}

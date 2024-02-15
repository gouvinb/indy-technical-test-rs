use actix_web::{delete, get, put, web, HttpResponse};

use promocode_models::data::promocode::Promocode;
use promocode_models::extensions::vec_restriction::RestrictionsExt;
use promocode_models::req::promocode_request::PromocodeRequest;

use crate::db::{db_delete_by_name, db_get_by_name, db_list, db_push};
use crate::open_weather_sdk;

/// Configure the promo code services.
///
/// This function should be called to configure the promo code-related services
/// in a [ServiceConfig]. It adds the necessary routes to handle the `GET` and
/// `PUT` requests for promo codes. If the code is compiled with debug
/// assertions enabled, it also adds the route for deleting promo codes.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the [ServiceConfig] where the routes will be added.
pub fn promocode_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_promocode);
    cfg.service(put_promocode);
    if cfg!(debug_assertions) {
        cfg.service(delete_promocode);
    }
}

/// Handler for validate a [PromocodeRequest].
///
/// This async function takes a [PromocodeRequest] in JSON format as input and
/// retrieves the corresponding [Promocode] from the database by calling
/// [db_get_by_name]. It then checks if the [Promocode] has any restrictions and
/// whether the request satisfies those restrictions.
///
/// If the promocode exists and the request satisfies any restrictions, it
/// returns an HTTP 200 response with the accepted [Promocode] in the body. If
/// the request does not satisfy the restrictions or the promocode does not
/// exist, it returns an HTTP 400 error with a [BadRequest] response.
///
/// # Arguments
///
/// - `promocode_req_json`: JSON payload containing the [PromocodeRequest]
///   details.
///
/// # Returns
///
/// An [HttpResponse] object that may contain a [PromocodeAccepted] or a
/// [BadRequest] with [PromocodeDenied].
///
/// # Errors
///
/// This function might return an error if there's a problem with:
///
/// - Retrieving the requested [Promocode] from the database.
/// - Checking the restrictions of the [Promocode].
/// - Generating a response with the provided [Promocode].
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

/// Handler for creating a new [Promocode].
///
/// # Arguments
///
/// - `promocode_json`: JSON payload containing the [Promocode] details.
///
/// # Returns
///
/// This function returns an [HttpResponse] indicating the status of the
/// operation.
///
/// - If a [Promocode] with the same id or name already exists in the database,
///   it returns a [HttpResponse::BadRequest()] response with an error message.
/// - If the [Promocode] was successfully added to the database, it returns an
///   [Ok] response with an empty JSON payload.
/// - If an error occurred while adding the [Promocode] to the database, it
///   returns a [HttpResponse::BadRequest()] response with the error message.
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

/// Handler for delete a [Promocode].
///
/// # Parameters
///
/// - `name`: The name of the [Promocode] to be deleted.
///
/// # Returns
///
/// An [HttpResponse] with a status code of 200 if the [Promocode] was
/// successfully deleted from the database. The response body is an empty JSON object.
#[delete("/promocode")]
pub async fn delete_promocode(name: web::Json<String>) -> HttpResponse {
    db_delete_by_name(name.to_owned());
    HttpResponse::Ok().json("")
}

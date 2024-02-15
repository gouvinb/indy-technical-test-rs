use std::sync::{Mutex, MutexGuard, OnceLock};

use log::error;
use ntex::web::types::Json;
use openweather_sdk::{Language, OpenWeather, Units};
use promocode_models::req::promocode_request::PromocodeRequest;
use std::error::Error;

static OPEN_WEATHER: OnceLock<Mutex<OpenWeather>> = OnceLock::new();

/// Initializes the Open Weather SDK with the specified API key.
///
/// # Returns
///
/// Returns a [Result] with an [Error] message in case the Open Weather SDK is
/// already initialized, or [Ok] indicating successful initialization.
pub fn init_open_weather_sdk(api_key: String) -> Result<(), String /* Error */> {
    match OPEN_WEATHER.set(Mutex::new(OpenWeather::new(api_key, Units::Metric, Language::default()))) {
        Ok(_) => Ok(()),
        Err(_) => Err("Open Weather SDK already initialized.".to_string()),
    }
}

/// Opens a weather instance.
///
/// # Returns
///
/// Returns a [Result] with the weather instance if successful, or an error
/// message if the Open Weather SDK is not initialized or if there was an error.
pub fn open_weather_instance() -> Result<MutexGuard<'static, OpenWeather>, String /* Error */> {
    match OPEN_WEATHER.get() {
        None => Err("Open Wether SDK must be initialized.".to_string()),
        Some(instance) => match instance.lock() {
            Ok(openweather_sdk) => Ok(openweather_sdk),
            Err(err) => Err(err.to_string()),
        },
    }
}

/// Retrieves the current weather and temperature for a given town specified in
/// the `promocode_req_json` argument.
///
/// # Arguments
///
/// - `promocode_req_json` - A reference to the JSON object containing the
///   [PromocodeRequest]
///
/// # Returns
///
/// An [Option] containing a tuple of the weather description (in lowercase) and
/// temperature if successful or returns [None] otherwise.
pub async fn get_current_meteo_and_temp(promocode_req_json: &Json<PromocodeRequest>) -> Option<(String, f64)> {
    let open_weather_instance = match open_weather_instance() {
        Ok(guard) => guard,
        Err(_) => return None,
    }
    .clone();

    let geocoding_result = open_weather_instance
        .geocoding
        .get_geocoding(promocode_req_json.arguments.clone().meteo.town.as_str(), None, None, 1)
        .await;

    #[allow(clippy::type_complexity)]
    let log_and_return_none: fn(Box<dyn Error>) -> Option<(String, f64)> = |err| {
        error!("{}", err);
        None
    };

    let weather_and_temp = match geocoding_result {
        Ok(geocoding_vec) if !geocoding_vec.is_empty() => {
            let first_geocoding = &geocoding_vec[0];
            match open_weather_instance.forecast.call(first_geocoding.lat, first_geocoding.lon, 1).await {
                Ok(forecast) if forecast.list.first().is_some_and(|data| data.weather.first().is_some()) => forecast.list.first().map(|first_data| {
                    (
                        first_data.weather.first().unwrap().main.to_lowercase().clone(),
                        forecast.list.first().unwrap().main.temp,
                    )
                }),
                Ok(forecast) => {
                    error!("No weather found!: {}", forecast);
                    None
                },
                Err(err) => log_and_return_none(err),
            }
        },
        Ok(_) => {
            error!("No location found!");
            None
        },
        Err(err) => log_and_return_none(err),
    };
    weather_and_temp
}

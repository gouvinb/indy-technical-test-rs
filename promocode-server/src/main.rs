use clap::Parser;
use log::{error, info, warn};

use promocode_server::cli::{Cli, ENV_VAR_NAME_OPEN_WEATHER_MAP_API_KEY};
use promocode_server::open_weather_sdk::init_open_weather_sdk;
use promocode_server::server;

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    if cli.open_weather_map_api_key.is_empty() {
        warn!(
            "{} environment variable is empty or not exist. So all weather restrictions will return false.",
            ENV_VAR_NAME_OPEN_WEATHER_MAP_API_KEY
        );
    } else {
        match init_open_weather_sdk(cli.open_weather_map_api_key) {
            Ok(_) => info!("{} environment variable initialized.", ENV_VAR_NAME_OPEN_WEATHER_MAP_API_KEY),
            Err(err) => error!("{}", err),
        };
    }

    match server::serve(cli.host, cli.port, cli.debug) {
        Ok(_) => {},
        Err(err) => {
            error!("{}", err)
        },
    }
}

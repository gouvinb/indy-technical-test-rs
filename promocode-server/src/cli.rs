use clap::Parser;

pub const ENV_VAR_NAME_OPEN_WEATHER_MAP_API_KEY: &str = "OPEN_WEATHER_MAP_API_KEY";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Hostname to operate on
    #[arg(long, value_name = "HOSTNAME", default_value = "127.0.0.1")]
    pub host: String,

    /// Port to operate on
    #[arg(long, value_name = "PORT", default_value_t = 8080)]
    pub port: u16,

    // /// Turn debugging information on
    // #[arg(short, long, default_value_t = false)]
    // pub debug: bool,

    /// Open Weather Map API key.
    ///
    /// Quota:
    ///
    ///  - `60` calls/minute
    ///  - `1_000_000` calls/month
    #[arg(env = ENV_VAR_NAME_OPEN_WEATHER_MAP_API_KEY, default_value = "")]
    pub open_weather_map_api_key: String,
}

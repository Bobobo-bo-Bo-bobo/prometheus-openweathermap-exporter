use crate::config;
use crate::constants;
use crate::http;
use crate::openweathermap;

use lazy_static::lazy_static;
use log::{debug, error};
use prometheus::{GaugeVec, IntGaugeVec, Opts, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref TEMPERATURE: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_TEMP_NAME, constants::METRIC_TEMP_HELP),
        &["name", "country"],
    )
    .unwrap();
    pub static ref TEMPERATURE_FEELS_LIKE: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_TEMP_FEELS_LIKE_NAME,
            constants::METRIC_TEMP_FEELS_LIKE_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref TEMPERATURE_MIN: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_TEMP_MIN_NAME,
            constants::METRIC_TEMP_MIN_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref TEMPERATURE_MAX: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_TEMP_MAX_NAME,
            constants::METRIC_TEMP_MAX_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref PRESSURE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_PRESSURE_NAME,
            constants::METRIC_PRESSURE_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref HUMIDITY: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_HUMIDITY_NAME,
            constants::METRIC_HUMIDITY_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref WIND_SPEED: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_WIND_SPEED_NAME,
            constants::METRIC_WIND_SPEED_HELP,
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref WIND_GUST: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_WIND_GUST_NAME,
            constants::METRIC_WIND_GUST_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref WIND_DIRECTION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_WIND_DIRECTION_NAME,
            constants::METRIC_WIND_DIRECTION_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
}

pub fn register() {
    REGISTRY
        .register(Box::new(TEMPERATURE_FEELS_LIKE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(TEMPERATURE_MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(TEMPERATURE_MAX.clone()))
        .unwrap();
    REGISTRY.register(Box::new(TEMPERATURE.clone())).unwrap();
    REGISTRY.register(Box::new(PRESSURE.clone())).unwrap();
    REGISTRY.register(Box::new(HUMIDITY.clone())).unwrap();
    REGISTRY.register(Box::new(WIND_SPEED.clone())).unwrap();
    REGISTRY.register(Box::new(WIND_GUST.clone())).unwrap();
    REGISTRY.register(Box::new(WIND_DIRECTION.clone())).unwrap();
}

fn update_metrics(cfg: &config::Configuration) {
    let cfg = cfg.clone();
    let timeout = cfg.timeout.unwrap_or(constants::HTTP_CLIENT_TIMEOUT);
    let mut client = match http::build_client(timeout) {
        Ok(v) => v,
        Err(e) => panic!("Can't build HTTP client structure: {}", e),
    };
    let units = cfg
        .units
        .unwrap_or_else(|| constants::DEFAULT_OWM_UNITS.to_string());
    for location in cfg.locations {
        let url = format!(
            "{}?q={}&units={}&APPID={}",
            constants::OWM_URL,
            location,
            units,
            cfg.api_key
        );

        debug!("Requesting data from {}", url);
        let reply = match http::get(&mut client, &url) {
            Ok(v) => v,
            Err(e) => {
                error!("Can't fetch weather data for {}: {}", location, e);
                continue;
            }
        };

        let data: openweathermap::OpenWeatherMap = match serde_json::from_str(&reply) {
            Ok(v) => v,
            Err(e) => {
                error!("Can't parse result for {} as JSON: {}", location, e);
                continue;
            }
        };
        debug!(
            "Setting openweathermap_temperature_celsius {} {} -> {}",
            data.name, data.sys.country, data.main.temp
        );
        TEMPERATURE
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.main.temp);

        debug!(
            "Setting openweathermap_apparent_temperature_celsius {} {} -> {}",
            data.name, data.sys.country, data.main.feels_like
        );
        TEMPERATURE_FEELS_LIKE
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.main.feels_like);

        debug!(
            "Setting openweathermap_minimal_temperature_celsius {} {} -> {}",
            data.name, data.sys.country, data.main.temp_min
        );
        TEMPERATURE_MIN
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.main.temp_min);

        debug!(
            "Setting openweathermap_maximal_temperature_celsius {} {} -> {}",
            data.name, data.sys.country, data.main.temp_max
        );
        TEMPERATURE_MAX
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.main.temp_max);

        debug!(
            "Setting openweathermap_pressure_pascal {} {} -> {}",
            data.name,
            data.sys.country,
            100 * data.main.pressure
        );
        PRESSURE
            .with_label_values(&[&data.name, &data.sys.country])
            .set(100 * data.main.pressure as i64);

        debug!(
            "Setting openweathermap_humidity_percent {} {} -> {}",
            data.name,
            data.sys.country,
            data.main.humidity as f64 / 100.0
        );
        HUMIDITY
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.main.humidity as f64 / 100.0);

        debug!(
            "Setting openweathermap_wind_speed_kilometers_per_hour {} {} -> {}",
            data.name, data.sys.country, data.wind.speed
        );
        WIND_SPEED
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.wind.speed);
        debug!(
            "Setting openweathermap_wind_gust_speed_kilometers_per_hour {} {} -> {}",
            data.name, data.sys.country, data.wind.gust
        );
        WIND_GUST
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.wind.gust);
        debug!(
            "Setting openweathermap_wind_direction_degree {} {} -> {}",
            data.name, data.sys.country, data.wind.deg
        );
        WIND_DIRECTION
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.wind.deg as i64);
    }
}

pub fn serve_metrics(cfg: &config::Configuration) -> String {
    update_metrics(cfg);

    let encoder = prometheus::TextEncoder::new();
    let mut buffer = String::new();

    if let Err(e) = encoder.encode_utf8(&REGISTRY.gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    }

    if let Err(e) = encoder.encode_utf8(&prometheus::gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    };
    buffer
}

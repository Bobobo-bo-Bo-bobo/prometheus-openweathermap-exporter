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
    pub static ref CLOUD: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_CLOUD_NAME, constants::METRIC_CLOUD_HELP,),
        &["name", "country"],
    )
    .unwrap();
    pub static ref RAIN_1H: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_RAIN_1H_NAME,
            constants::METRIC_RAIN_1H_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref RAIN_3H: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_RAIN_3H_NAME,
            constants::METRIC_RAIN_3H_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref SNOW_1H: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_SNOW_1H_NAME,
            constants::METRIC_SNOW_1H_HELP
        ),
        &["name", "country"],
    )
    .unwrap();
    pub static ref SNOW_3H: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_SNOW_3H_NAME,
            constants::METRIC_SNOW_3H_HELP
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
    REGISTRY.register(Box::new(CLOUD.clone())).unwrap();
    REGISTRY.register(Box::new(RAIN_1H.clone())).unwrap();
    REGISTRY.register(Box::new(RAIN_3H.clone())).unwrap();
    REGISTRY.register(Box::new(SNOW_1H.clone())).unwrap();
    REGISTRY.register(Box::new(SNOW_3H.clone())).unwrap();
}

fn update_metrics(cfg: &config::Configuration) {
    let cfg = cfg.clone();
    let timeout = cfg.timeout.unwrap_or(constants::HTTP_CLIENT_TIMEOUT);
    let mut client = match http::build_client(timeout) {
        Ok(v) => v,
        Err(e) => panic!("Can't build HTTP client structure: {}", e),
    };
    for location in cfg.locations {
        let url = format!(
            "{}?q={}&units={}&APPID={}",
            constants::OWM_URL,
            location,
            constants::DEFAULT_OWM_UNITS,
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

        if let Some(gust) = data.wind.gust {
            debug!(
                "Setting openweathermap_wind_gust_speed_kilometers_per_hour {} {} -> {}",
                data.name, data.sys.country, gust
            );
            WIND_GUST
                .with_label_values(&[&data.name, &data.sys.country])
                .set(gust);
        }

        debug!(
            "Setting openweathermap_wind_direction_degree {} {} -> {}",
            data.name, data.sys.country, data.wind.deg
        );
        WIND_DIRECTION
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.wind.deg as i64);
        debug!(
            "Setting openweathermap_cloud_coverage_percent {} {} -> {}",
            data.name,
            data.sys.country,
            data.clouds.all as f64 / 100.0
        );
        CLOUD
            .with_label_values(&[&data.name, &data.sys.country])
            .set(data.clouds.all as f64 / 100.0);

        if let Some(rain) = data.rain {
            if let Some(one_h) = rain.one_h {
                debug!(
                    "Setting openweathermap_rain_precipation_last_hour_millimeter {} {} -> {}",
                    data.name, data.sys.country, one_h
                );
                RAIN_1H
                    .with_label_values(&[&data.name, &data.sys.country])
                    .set(one_h);
            }
            if let Some(three_h) = rain.three_h {
                debug!(
                    "Setting openweathermap_rain_precipation_last_three_hours_millimeter {} {} -> {}",
                    data.name,
                    data.sys.country,
                    three_h
                );
                RAIN_3H
                    .with_label_values(&[&data.name, &data.sys.country])
                    .set(three_h);
            }
        }

        if let Some(snow) = data.snow {
            if let Some(one_h) = snow.one_h {
                debug!(
                    "Setting openweathermap_snow_precipation_last_hour_millimeter {} {} -> {}",
                    data.name, data.sys.country, one_h
                );
                SNOW_1H
                    .with_label_values(&[&data.name, &data.sys.country])
                    .set(one_h);
            }
            if let Some(three_h) = snow.three_h {
                debug!(
                    "Setting openweathermap_snow_precipation_last_three_hours_millimeter {} {} -> {}",
                    data.name,
                    data.sys.country,
                    three_h
                );
                SNOW_3H
                    .with_label_values(&[&data.name, &data.sys.country])
                    .set(three_h);
            }
        }
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

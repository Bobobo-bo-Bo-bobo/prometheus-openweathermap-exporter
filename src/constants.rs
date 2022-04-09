pub const NAME: &str = "prometheus-openweathermap-exporter";
pub const VERSION: &str = "0.1.0-20220408";
pub const DEFAULT_PROMETHEUS_ADDRESS: &str = "localhost:9943";
pub const REPO_URL: &str = "https://ypbind.de/cgit/prometheus-openweathermap-exporter/";

pub fn generate_default_user_agent() -> String {
    format!("{}/{} ({})", NAME, VERSION, REPO_URL)
}
pub const ROOT_HTML: &str = "<html>\n<head><title>OpenWeatherMap exporter</title></head>\n<body>\n<h1>OpenWeatherMap exporter</h1>\n<p><a href=\"/metrics\">Metrics</a></p>\n</body>\n</html>\n";
pub const METRICS_PATH: &str = "/metrics";
pub const HTTP_CLIENT_TIMEOUT: u64 = 15;
pub const DEFAULT_OWM_UNITS: &str = "metric";
pub const OWM_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

pub const METRIC_TEMP_NAME: &str = "openweathermap_temperature_celsius";
pub const METRIC_TEMP_HELP: &str = "Temperature";
pub const METRIC_TEMP_FEELS_LIKE_NAME: &str = "openweathermap_apparent_temperature_celsius";
pub const METRIC_TEMP_FEELS_LIKE_HELP: &str = "Apparent temperature";
pub const METRIC_TEMP_MIN_NAME: &str = "openweathermap_minimal_temperature_celsius";
pub const METRIC_TEMP_MIN_HELP: &str = "Minimal temperature";
pub const METRIC_TEMP_MAX_NAME: &str = "openweathermap_maximal_temperature_celsius";
pub const METRIC_TEMP_MAX_HELP: &str = "Maximal temperature";
pub const METRIC_PRESSURE_NAME: &str = "openweathermap_pressure_pascal";
pub const METRIC_PRESSURE_HELP: &str = "Air pressure";
pub const METRIC_HUMIDITY_NAME: &str = "openweathermap_humidity_percent";
pub const METRIC_HUMIDITY_HELP: &str = "Relatrive humidity";
pub const METRIC_WIND_SPEED_NAME: &str = "openweathermap_wind_speed_meters_per_hour";
pub const METRIC_WIND_SPEED_HELP: &str = "Wind speed";
pub const METRIC_WIND_GUST_NAME: &str = "openweathermap_wind_gust_speed_meters_per_hour";
pub const METRIC_WIND_GUST_HELP: &str = "Wind gust speed";
pub const METRIC_WIND_DIRECTION_NAME: &str = "openweathermap_wind_direction_degree";
pub const METRIC_WIND_DIRECTION_HELP: &str = "Wind direction";

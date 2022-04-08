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

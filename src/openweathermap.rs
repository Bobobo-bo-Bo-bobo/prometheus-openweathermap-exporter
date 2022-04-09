use serde::Deserialize;

// Documentation of the data format -> https://openweathermap.org/weather-data#current
#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMap {
    pub base: String,
    pub clouds: OpenWeatherMapClouds,
    pub cod: u8,
    pub coord: OpenWeatherMapCoordinates,
    pub dt: i64,
    pub id: u64,
    pub main: OpenWeatherMapMain,
    pub name: String,
    pub rain: Option<OpenWeatherMapRainOrSnow>,
    pub snow: Option<OpenWeatherMapRainOrSnow>,
    pub sys: OpenWeatherMapSys,
    pub timezone: i32,
    pub visibility: i64,
    pub weather: Vec<OpenWeatherMapWeather>,
    pub wind: OpenWeatherMapWind,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapCoordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapWeather {
    pub description: String,
    pub icon: String,
    pub id: i64,
    pub main: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapMain {
    pub feels_like: f64,
    pub grnd_level: Option<i32>,
    pub humidity: u8,
    pub pressure: u32,
    pub sea_level: Option<i32>,
    pub temp: f64,
    pub temp_max: f64,
    pub temp_min: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapWind {
    pub deg: u16,
    pub gust: Option<f64>,
    pub speed: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapRainOrSnow {
    #[serde(rename = "1h")]
    pub one_h: Option<f64>,
    #[serde(rename = "3h")]
    pub three_h: Option<f64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapClouds {
    pub all: u8,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapSys {
    pub country: String,
    pub id: i64,
    #[serde(rename = "type")]
    pub owm_type: i32,
    pub sunrise: u64,
    pub sunset: u64,
}

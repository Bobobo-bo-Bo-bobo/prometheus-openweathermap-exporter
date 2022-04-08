use serde::Deserialize;

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
    pub rain: OpenWeatherMapRain,
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
    pub grnd_level: i32,
    pub humidity: u8,
    pub pressure: i32,
    pub sea_level: i32;
    pub temp: f64,
    pub temp_max: f64,
    pub temp_min: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapWind {
    pub deg: u16,
    pub gust: f64,
    pub speed: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenWeatherMapRain {
    #[serde(rename = "1h")]
    pub one_h: u64,
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

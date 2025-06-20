use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenMeteoWeather {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "generationtime_ms")]
    pub generationtime_ms: f64,
    #[serde(rename = "utc_offset_seconds")]
    pub utc_offset_seconds: i64,
    pub timezone: String,
    #[serde(rename = "timezone_abbreviation")]
    pub timezone_abbreviation: String,
    pub elevation: f64,
    #[serde(rename = "current_units")]
    pub current_units: CurrentUnits,
    pub current: Current,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUnits {
    pub time: String,
    pub interval: String,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: String,
    #[serde(rename = "relative_humidity_2m")]
    pub relative_humidity_2m: String,
    #[serde(rename = "apparent_temperature")]
    pub apparent_temperature: String,
    #[serde(rename = "is_day")]
    pub is_day: String,
    pub precipitation: String,
    #[serde(rename = "cloud_cover")]
    pub cloud_cover: String,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed_10m: String,
    #[serde(rename = "wind_direction_10m")]
    pub wind_direction_10m: String,
    #[serde(rename = "wind_gusts_10m")]
    pub wind_gusts_10m: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub time: String,
    pub interval: i64,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: f64,
    #[serde(rename = "relative_humidity_2m")]
    pub relative_humidity_2m: i64,
    #[serde(rename = "apparent_temperature")]
    pub apparent_temperature: f64,
    #[serde(rename = "is_day")]
    pub is_day: i64,
    pub precipitation: f64,
    #[serde(rename = "cloud_cover")]
    pub cloud_cover: i64,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed_10m: f64,
    #[serde(rename = "wind_direction_10m")]
    pub wind_direction_10m: i64,
    #[serde(rename = "wind_gusts_10m")]
    pub wind_gusts_10m: f64,
}

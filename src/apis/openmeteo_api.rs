use crate::datamodels::openmeteo_data::OpenMeteoWeather;
use log::info;

pub async fn fetch_weather_async(lat: f32, lon: f32) -> OpenMeteoWeather {
    info!("Fetching weather data for coordinates: ({}, {})", lat, lon);

    let api_endpoint = format!(
        // line is too long, so we use concat! to split it
        concat!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}",
            "&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,",
            "precipitation,cloud_cover,wind_speed_10m,wind_direction_10m,wind_gusts_10m",
            "&timezone=Asia%2FBangkok&forecast_days=1"
        ),
        lat, lon
    );

    let response = reqwest::get(&api_endpoint)
        .await
        .expect("Failed to fetch weather data");

    if response.status().is_success() {
        let weather_data: OpenMeteoWeather =
            response.json().await.expect("Failed to parse weather data");
        info!(
            "Weather data fetched successfully for coordinates: ({}, {})",
            lat, lon
        );
        weather_data
    } else {
        panic!("Failed to fetch weather data: {}", response.status());
    }
}

pub fn format_human_readable(weather: &OpenMeteoWeather) -> String {
    format!(
        "It's currently {}{} with {}{} relative humidity. The wind is {}{} at {}{} with {}{} cloud cover.",
        weather.current.temperature_2m,
        weather.current_units.temperature_2m,
        weather.current.relative_humidity_2m,
        weather.current_units.relative_humidity_2m,
        weather.current.wind_speed_10m,
        weather.current_units.wind_speed_10m,
        weather.current.wind_direction_10m,
        weather.current_units.wind_direction_10m,
        weather.current.cloud_cover,
        weather.current_units.cloud_cover
    )
}

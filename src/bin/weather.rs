use env_logger;
use log::info;
use openmeteo_weather::apis::ip_api::{fetch_user_ip_async, get_lat_lon_from_ip_async};
use openmeteo_weather::apis::openmeteo_api::{fetch_weather_async, format_human_readable};

#[tokio::main]
async fn main() {
    env_logger::init();

    // Fetch user's IP address
    let user_ip = fetch_user_ip_async()
        .await
        .expect("Failed to fetch user IP");
    info!("User's IP address: {}", user_ip);

    // Get latitude and longitude from the user's IP address
    let (lat, lon) = get_lat_lon_from_ip_async(&user_ip)
        .await
        .expect("Failed to get latitude and longitude from IP");
    info!("User's location: Latitude: {}, Longitude: {}", lat, lon);

    // Fetch weather data for the user's location
    let weather = fetch_weather_async(lat as f32, lon as f32).await;

    // Format and print the weather data
    let human_readable_weather = format_human_readable(&weather);
    println!("{}", human_readable_weather);
}

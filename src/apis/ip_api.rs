use crate::datamodels::ip_data::FreeIPApiData;
use log::info;
use reqwest;

pub async fn fetch_user_ip_async() -> Result<String, &'static str> {
    let response = reqwest::get("https://api.ipify.org")
        .await
        .map_err(|_| "Failed to fetch user IP")?;
    if response.status().is_success() {
        let ip: String = response
            .text()
            .await
            .map_err(|_| "Failed to read IP response")?;
        info!("User IP fetched successfully: {}", ip);
        Ok(ip)
    } else {
        Err("Failed to fetch user IP")
    }
}

pub async fn get_lat_lon_from_ip_async(ip: &str) -> Result<(f64, f64), &'static str> {
    let api_url = format!("https://free.freeipapi.com/api/json/{}", ip);
    let response = reqwest::get(&api_url)
        .await
        .map_err(|_| "Failed to fetch IP data")?;
    if response.status().is_success() {
        let ip_data: FreeIPApiData = response.json().await.expect("Failed to parse IP data");

        info!("IP data fetched successfully for IP: {}", ip);

        Ok((ip_data.latitude, ip_data.longitude))
    } else {
        Err("Failed to fetch IP data")
    }
}

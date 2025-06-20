use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeIPApiData {
    pub ip_version: i64,
    pub ip_address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country_name: String,
    pub country_code: String,
    pub capital: String,
    pub phone_codes: Vec<i64>,
    pub time_zones: Vec<String>,
    pub city_name: String,
    pub region_name: String,
    pub continent: String,
    pub continent_code: String,
    pub currencies: Vec<String>,
    pub languages: Vec<String>,
    pub asn: String,
    pub asn_organization: String,
}

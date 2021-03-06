/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricePerTime {
    /// Name of the Location the price is for
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "price_hourly")]
    pub price_hourly: crate::models::Price,
    #[serde(rename = "price_monthly")]
    pub price_monthly: crate::models::Price,
}

impl PricePerTime {
    pub fn new(location: String, price_hourly: crate::models::Price, price_monthly: crate::models::Price) -> PricePerTime {
        PricePerTime {
            location,
            price_hourly,
            price_monthly,
        }
    }
}



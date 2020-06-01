/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPricesResponse {
    #[serde(rename = "pricing")]
    pub pricing: crate::models::ListPricesResponsePricing,
}

impl ListPricesResponse {
    pub fn new(pricing: crate::models::ListPricesResponsePricing) -> ListPricesResponse {
        ListPricesResponse {
            pricing,
        }
    }
}


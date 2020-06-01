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
pub struct CreateNetworkResponse {
    #[serde(rename = "network")]
    pub network: crate::models::Network,
}

impl CreateNetworkResponse {
    pub fn new(network: crate::models::Network) -> CreateNetworkResponse {
        CreateNetworkResponse {
            network,
        }
    }
}


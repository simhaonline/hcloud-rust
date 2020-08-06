/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceNetworkResponse : Response to PUT https://api.hetzner.cloud/v1/networks/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceNetworkResponse {
    #[serde(rename = "network")]
    pub network: crate::models::ReplaceNetworkResponseNetwork,
}

impl ReplaceNetworkResponse {
    /// Response to PUT https://api.hetzner.cloud/v1/networks/{id}
    pub fn new(network: crate::models::ReplaceNetworkResponseNetwork) -> ReplaceNetworkResponse {
        ReplaceNetworkResponse {
            network,
        }
    }
}



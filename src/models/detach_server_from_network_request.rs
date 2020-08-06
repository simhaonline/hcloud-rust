/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DetachServerFromNetworkRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_from_network



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetachServerFromNetworkRequest {
    /// ID of an existing network to detach the Server from
    #[serde(rename = "network")]
    pub network: i32,
}

impl DetachServerFromNetworkRequest {
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_from_network
    pub fn new(network: i32) -> DetachServerFromNetworkRequest {
        DetachServerFromNetworkRequest {
            network,
        }
    }
}



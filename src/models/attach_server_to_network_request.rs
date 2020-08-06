/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AttachServerToNetworkRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_to_network



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachServerToNetworkRequest {
    /// ID of an existing network to attach the Server to
    #[serde(rename = "network")]
    pub network: i32,
    /// IP to request to be assigned to this Server; if you do not provide this then you will be auto assigned an IP address
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Additional IPs to be assigned to this Server
    #[serde(rename = "alias_ips", skip_serializing_if = "Option::is_none")]
    pub alias_ips: Option<Vec<serde_json::Value>>,
}

impl AttachServerToNetworkRequest {
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_to_network
    pub fn new(network: i32) -> AttachServerToNetworkRequest {
        AttachServerToNetworkRequest {
            network,
            ip: None,
            alias_ips: None,
        }
    }
}



/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateFloatingIpResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFloatingIpResponse {
    #[serde(rename = "floating_ip")]
    pub floating_ip: crate::models::FloatingIp,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::models::Action>,
}

impl CreateFloatingIpResponse {
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips
    pub fn new(floating_ip: crate::models::FloatingIp) -> CreateFloatingIpResponse {
        CreateFloatingIpResponse {
            floating_ip,
            action: None,
        }
    }
}



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
pub struct ListFloatingIpsResponse {
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<crate::models::FloatingIp>,
}

impl ListFloatingIpsResponse {
    pub fn new(floating_ips: Vec<crate::models::FloatingIp>) -> ListFloatingIpsResponse {
        ListFloatingIpsResponse {
            floating_ips,
        }
    }
}


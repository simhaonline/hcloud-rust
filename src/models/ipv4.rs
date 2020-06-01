/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Ipv4 : IP address (v4) and its reverse DNS entry of this Server



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv4 {
    /// IP address (v4) of this Server
    #[serde(rename = "ip")]
    pub ip: String,
    /// If the IP is blocked by our anti abuse dept
    #[serde(rename = "blocked")]
    pub blocked: bool,
    /// Reverse DNS PTR entry for the IPv4 addresses of this Server
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: String,
}

impl Ipv4 {
    /// IP address (v4) and its reverse DNS entry of this Server
    pub fn new(ip: String, blocked: bool, dns_ptr: String) -> Ipv4 {
        Ipv4 {
            ip,
            blocked,
            dns_ptr,
        }
    }
}


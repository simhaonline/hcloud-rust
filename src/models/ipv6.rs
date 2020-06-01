/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Ipv6 : IPv6 network assigned to this Server and its reverse DNS entry



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv6 {
    /// IP address (v4) of this Server
    #[serde(rename = "ip")]
    pub ip: String,
    /// If the IP is blocked by our anti abuse dept
    #[serde(rename = "blocked")]
    pub blocked: bool,
    /// Reverse DNS PTR entries for the IPv6 addresses of this Server, `null` by default
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: Option<Vec<crate::models::DnsPtr>>,
}

impl Ipv6 {
    /// IPv6 network assigned to this Server and its reverse DNS entry
    pub fn new(ip: String, blocked: bool, dns_ptr: Option<Vec<crate::models::DnsPtr>>) -> Ipv6 {
        Ipv6 {
            ip,
            blocked,
            dns_ptr,
        }
    }
}


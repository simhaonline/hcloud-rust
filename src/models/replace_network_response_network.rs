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
pub struct ReplaceNetworkResponseNetwork {
    /// ID of the Network
    #[serde(rename = "id")]
    pub id: i32,
    /// Name of the network
    #[serde(rename = "name")]
    pub name: String,
    /// IPv4 prefix of the whole Network
    #[serde(rename = "ip_range")]
    pub ip_range: String,
    /// Array subnets allocated in this Network
    #[serde(rename = "subnets")]
    pub subnets: Vec<crate::models::SubnetWithGateway>,
    /// Array of routes set in this Network
    #[serde(rename = "routes")]
    pub routes: Vec<crate::models::Route>,
    /// Array of IDs of Servers attached to this Network
    #[serde(rename = "servers")]
    pub servers: Vec<i32>,
    #[serde(rename = "protection")]
    pub protection: crate::models::NetworkProtection,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::models::ReplaceFloatingIpResponseFloatingIpLabels>,
    /// Point in time when the Network was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
}

impl ReplaceNetworkResponseNetwork {
    pub fn new(id: i32, name: String, ip_range: String, subnets: Vec<crate::models::SubnetWithGateway>, routes: Vec<crate::models::Route>, servers: Vec<i32>, protection: crate::models::NetworkProtection, created: String) -> ReplaceNetworkResponseNetwork {
        ReplaceNetworkResponseNetwork {
            id,
            name,
            ip_range,
            subnets,
            routes,
            servers,
            protection,
            labels: None,
            created,
        }
    }
}


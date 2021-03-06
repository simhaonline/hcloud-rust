/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceLoadBalancerResponseLoadBalancer {
    /// ID of the Load Balancer
    #[serde(rename = "id")]
    pub id: i32,
    /// Name of the Load Balancer
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "public_net")]
    pub public_net: crate::models::LoadBalancerPublicNet,
    /// Private networks information
    #[serde(rename = "private_net")]
    pub private_net: Vec<serde_json::Value>,
    #[serde(rename = "location")]
    pub location: crate::models::Location,
    #[serde(rename = "load_balancer_type")]
    pub load_balancer_type: crate::models::LoadBalancerType,
    #[serde(rename = "protection")]
    pub protection: crate::models::LoadBalancerProtection,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Point in time when the Load Balancer was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// List of services that belong to this Load Balancer
    #[serde(rename = "services")]
    pub services: Vec<crate::models::Service>,
    /// List of targets that belong to this Load Balancer
    #[serde(rename = "targets")]
    pub targets: Vec<crate::models::Target>,
    #[serde(rename = "algorithm")]
    pub algorithm: crate::models::LoadBalancerAlgorithm,
    /// Outbound Traffic for the current billing period in bytes
    #[serde(rename = "outgoing_traffic")]
    pub outgoing_traffic: Option<i64>,
    /// Inbound Traffic for the current billing period in bytes
    #[serde(rename = "ingoing_traffic")]
    pub ingoing_traffic: Option<i64>,
    /// Free Traffic for the current billing period in bytes
    #[serde(rename = "included_traffic")]
    pub included_traffic: i64,
}

impl ReplaceLoadBalancerResponseLoadBalancer {
    pub fn new(id: i32, name: String, public_net: crate::models::LoadBalancerPublicNet, private_net: Vec<serde_json::Value>, location: crate::models::Location, load_balancer_type: crate::models::LoadBalancerType, protection: crate::models::LoadBalancerProtection, created: String, services: Vec<crate::models::Service>, targets: Vec<crate::models::Target>, algorithm: crate::models::LoadBalancerAlgorithm, outgoing_traffic: Option<i64>, ingoing_traffic: Option<i64>, included_traffic: i64) -> ReplaceLoadBalancerResponseLoadBalancer {
        ReplaceLoadBalancerResponseLoadBalancer {
            id,
            name,
            public_net,
            private_net,
            location,
            load_balancer_type,
            protection,
            labels: None,
            created,
            services,
            targets,
            algorithm,
            outgoing_traffic,
            ingoing_traffic,
            included_traffic,
        }
    }
}



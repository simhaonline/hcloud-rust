/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerAlgorithm : Algorithm of the Load Balancer



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerAlgorithm {
    /// Type of the algorithm
    #[serde(rename = "type")]
    pub _type: Type,
}

impl LoadBalancerAlgorithm {
    /// Algorithm of the Load Balancer
    pub fn new(_type: Type) -> LoadBalancerAlgorithm {
        LoadBalancerAlgorithm {
            _type,
        }
    }
}

/// Type of the algorithm
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "least_connections")]
    LeastConnections,
    #[serde(rename = "round_robin")]
    RoundRobin,
}


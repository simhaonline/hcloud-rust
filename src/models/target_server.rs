/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TargetServer : Server where the traffic should be routed through



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetServer {
    /// ID of the Server
    #[serde(rename = "id")]
    pub id: i32,
}

impl TargetServer {
    /// Server where the traffic should be routed through
    pub fn new(id: i32) -> TargetServer {
        TargetServer {
            id,
        }
    }
}



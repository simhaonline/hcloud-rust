/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RemoveTargetRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveTargetRequest {
    /// Type of the resource
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<crate::models::AddTargetRequestServer>,
    #[serde(rename = "label_selector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<crate::models::AddTargetRequestLabelSelector>,
}

impl RemoveTargetRequest {
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target
    pub fn new(_type: Type) -> RemoveTargetRequest {
        RemoveTargetRequest {
            _type,
            server: None,
            label_selector: None,
        }
    }
}

/// Type of the resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "label_selector")]
    LabelSelector,
    #[serde(rename = "server")]
    Server,
}


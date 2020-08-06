/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RemoveTargetResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveTargetResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl RemoveTargetResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target
    pub fn new(action: crate::models::Action) -> RemoveTargetResponse {
        RemoveTargetResponse {
            action,
        }
    }
}



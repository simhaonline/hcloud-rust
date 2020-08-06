/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AddServiceResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/add_service



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddServiceResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl AddServiceResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/add_service
    pub fn new(action: crate::models::Action) -> AddServiceResponse {
        AddServiceResponse {
            action,
        }
    }
}


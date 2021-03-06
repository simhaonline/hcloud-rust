/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResizeVolumeResponse : Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/resize



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResizeVolumeResponse {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::models::Action>,
}

impl ResizeVolumeResponse {
    /// Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/resize
    pub fn new() -> ResizeVolumeResponse {
        ResizeVolumeResponse {
            action: None,
        }
    }
}



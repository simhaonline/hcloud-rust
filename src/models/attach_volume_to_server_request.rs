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
pub struct AttachVolumeToServerRequest {
    /// ID of the Server the Volume will be attached to
    #[serde(rename = "server")]
    pub server: i32,
    /// Auto-mount the Volume after attaching it
    #[serde(rename = "automount", skip_serializing_if = "Option::is_none")]
    pub automount: Option<bool>,
}

impl AttachVolumeToServerRequest {
    pub fn new(server: i32) -> AttachVolumeToServerRequest {
        AttachVolumeToServerRequest {
            server,
            automount: None,
        }
    }
}


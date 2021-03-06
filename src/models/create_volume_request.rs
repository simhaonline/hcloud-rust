/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateVolumeRequest : Request for POST https://api.hetzner.cloud/v1/volumes



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVolumeRequest {
    /// Size of the Volume in GB
    #[serde(rename = "size")]
    pub size: f32,
    /// Name of the volume
    #[serde(rename = "name")]
    pub name: String,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Auto-mount Volume after attach. `server` must be provided.
    #[serde(rename = "automount", skip_serializing_if = "Option::is_none")]
    pub automount: Option<bool>,
    /// Format Volume after creation. One of: `xfs`, `ext4`
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Location to create the Volume in (can be omitted if Server is specified)
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Server to which to attach the Volume once it's created (Volume will be created in the same Location as the server)
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<f32>,
}

impl CreateVolumeRequest {
    /// Request for POST https://api.hetzner.cloud/v1/volumes
    pub fn new(size: f32, name: String) -> CreateVolumeRequest {
        CreateVolumeRequest {
            size,
            name,
            labels: None,
            automount: None,
            format: None,
            location: None,
            server: None,
        }
    }
}



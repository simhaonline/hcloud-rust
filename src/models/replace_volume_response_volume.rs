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
pub struct ReplaceVolumeResponseVolume {
    /// ID of the Volume
    #[serde(rename = "id")]
    pub id: i32,
    /// Point in time when the Volume was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// Name of the Volume
    #[serde(rename = "name")]
    pub name: String,
    /// ID of the Server the Volume is attached to, null if it is not attached at all
    #[serde(rename = "server")]
    pub server: Option<i32>,
    #[serde(rename = "location")]
    pub location: crate::models::Location,
    /// Size in GB of the Volume
    #[serde(rename = "size")]
    pub size: f32,
    /// Device path on the file system for the Volume
    #[serde(rename = "linux_device")]
    pub linux_device: String,
    #[serde(rename = "protection")]
    pub protection: crate::models::ReplaceVolumeResponseVolumeProtection,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::models::ReplaceFloatingIpResponseFloatingIpLabels>,
    /// Current status of the Volume
    #[serde(rename = "status")]
    pub status: Status,
    /// Filesystem of the Volume if formatted on creation, null if not formatted on creation
    #[serde(rename = "format")]
    pub format: Option<String>,
}

impl ReplaceVolumeResponseVolume {
    pub fn new(id: i32, created: String, name: String, server: Option<i32>, location: crate::models::Location, size: f32, linux_device: String, protection: crate::models::ReplaceVolumeResponseVolumeProtection, status: Status, format: Option<String>) -> ReplaceVolumeResponseVolume {
        ReplaceVolumeResponseVolume {
            id,
            created,
            name,
            server,
            location,
            size,
            linux_device,
            protection,
            labels: None,
            status,
            format,
        }
    }
}

/// Current status of the Volume
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "creating")]
    Creating,
}

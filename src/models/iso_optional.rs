/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IsoOptional : ISO Image that is attached to this Server. Null if no ISO is attached.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsoOptional {
    /// ID of the ISO
    #[serde(rename = "id")]
    pub id: i32,
    /// Unique identifier of the ISO. Only set for public ISOs
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Description of the ISO
    #[serde(rename = "description")]
    pub description: String,
    /// Type of the ISO
    #[serde(rename = "type")]
    pub _type: Type,
    /// ISO 8601 timestamp of deprecation, null if ISO is still available. After the deprecation time it will no longer be possible to attach the ISO to Servers.
    #[serde(rename = "deprecated")]
    pub deprecated: Option<String>,
}

impl IsoOptional {
    /// ISO Image that is attached to this Server. Null if no ISO is attached.
    pub fn new(id: i32, name: Option<String>, description: String, _type: Type, deprecated: Option<String>) -> IsoOptional {
        IsoOptional {
            id,
            name,
            description,
            _type,
            deprecated,
        }
    }
}

/// Type of the ISO
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}


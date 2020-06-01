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
pub struct Server {
    /// ID of Server
    #[serde(rename = "id")]
    pub id: i32,
    /// Name of the Server (must be unique per Project and a valid hostname as per RFC 1123)
    #[serde(rename = "name")]
    pub name: String,
    /// Status of the Server
    #[serde(rename = "status")]
    pub status: Status,
    /// Point in time when the Server was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "public_net")]
    pub public_net: crate::models::PublicNet,
    /// Private networks information
    #[serde(rename = "private_net")]
    pub private_net: Vec<serde_json::Value>,
    #[serde(rename = "server_type")]
    pub server_type: crate::models::ServerType,
    #[serde(rename = "datacenter")]
    pub datacenter: crate::models::Datacenter,
    #[serde(rename = "image")]
    pub image: Option<crate::models::ImageOptional>,
    #[serde(rename = "iso")]
    pub iso: Option<crate::models::IsoOptional>,
    /// True if rescue mode is enabled: Server will then boot into rescue system on next reboot
    #[serde(rename = "rescue_enabled")]
    pub rescue_enabled: bool,
    /// True if Server has been locked and is not available to user
    #[serde(rename = "locked")]
    pub locked: bool,
    /// Time window (UTC) in which the backup will run, or null if the backups are not enabled
    #[serde(rename = "backup_window")]
    pub backup_window: Option<String>,
    /// Outbound Traffic for the current billing period in bytes
    #[serde(rename = "outgoing_traffic")]
    pub outgoing_traffic: Option<i64>,
    /// Inbound Traffic for the current billing period in bytes
    #[serde(rename = "ingoing_traffic")]
    pub ingoing_traffic: Option<i64>,
    /// Free Traffic for the current billing period in bytes
    #[serde(rename = "included_traffic")]
    pub included_traffic: i64,
    #[serde(rename = "protection")]
    pub protection: crate::models::Protection,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: serde_json::Value,
    /// IDs of Volumes assigned to this Server
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<serde_json::Value>>,
}

impl Server {
    pub fn new(id: i32, name: String, status: Status, created: String, public_net: crate::models::PublicNet, private_net: Vec<serde_json::Value>, server_type: crate::models::ServerType, datacenter: crate::models::Datacenter, image: Option<crate::models::ImageOptional>, iso: Option<crate::models::IsoOptional>, rescue_enabled: bool, locked: bool, backup_window: Option<String>, outgoing_traffic: Option<i64>, ingoing_traffic: Option<i64>, included_traffic: i64, protection: crate::models::Protection, labels: serde_json::Value) -> Server {
        Server {
            id,
            name,
            status,
            created,
            public_net,
            private_net,
            server_type,
            datacenter,
            image,
            iso,
            rescue_enabled,
            locked,
            backup_window,
            outgoing_traffic,
            ingoing_traffic,
            included_traffic,
            protection,
            labels,
            volumes: None,
        }
    }
}

/// Status of the Server
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "migrating")]
    Migrating,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "rebuilding")]
    Rebuilding,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "unknown")]
    Unknown,
}

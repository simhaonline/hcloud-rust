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
pub struct ReplaceSshKeyResponseSshKey {
    /// ID of the SSH key
    #[serde(rename = "id")]
    pub id: i32,
    /// Name of the SSH key
    #[serde(rename = "name")]
    pub name: String,
    /// Fingerprint of public key
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Public key
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::models::ReplaceFloatingIpResponseFloatingIpLabels>,
    /// Point in time when the SSH key was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
}

impl ReplaceSshKeyResponseSshKey {
    pub fn new(id: i32, name: String, fingerprint: String, public_key: String, created: String) -> ReplaceSshKeyResponseSshKey {
        ReplaceSshKeyResponseSshKey {
            id,
            name,
            fingerprint,
            public_key,
            labels: None,
            created,
        }
    }
}


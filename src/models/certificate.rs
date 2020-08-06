/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Certificate : TLS/SSL Certificates prove the identity of a Server and are used to encrypt client traffic.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    /// ID of the Certificate
    #[serde(rename = "id")]
    pub id: i32,
    /// Unique name of the Certificate
    #[serde(rename = "name")]
    pub name: String,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Certificate and chain in PEM format, in order so that each record directly certifies the one preceding
    #[serde(rename = "certificate")]
    pub certificate: String,
    /// Point in time when the Certificate was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// Point in time when the Certificate becomes valid (in ISO-8601 format)
    #[serde(rename = "not_valid_before")]
    pub not_valid_before: String,
    /// Point in time when the Certificate stops being valid (in ISO-8601 format)
    #[serde(rename = "not_valid_after")]
    pub not_valid_after: String,
    /// Domains and subdomains covered by the Certificate
    #[serde(rename = "domain_names")]
    pub domain_names: Vec<String>,
    /// Fingerprint of the Certificate
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Resources currently using the Certificate.
    #[serde(rename = "used_by")]
    pub used_by: Vec<crate::models::Resource>,
}

impl Certificate {
    /// TLS/SSL Certificates prove the identity of a Server and are used to encrypt client traffic.
    pub fn new(id: i32, name: String, labels: ::std::collections::HashMap<String, String>, certificate: String, created: String, not_valid_before: String, not_valid_after: String, domain_names: Vec<String>, fingerprint: String, used_by: Vec<crate::models::Resource>) -> Certificate {
        Certificate {
            id,
            name,
            labels,
            certificate,
            created,
            not_valid_before,
            not_valid_after,
            domain_names,
            fingerprint,
            used_by,
        }
    }
}



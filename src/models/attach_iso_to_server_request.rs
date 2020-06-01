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
pub struct AttachIsoToServerRequest {
    /// ID or name of ISO to attach to the Server as listed in GET `/isos`
    #[serde(rename = "iso")]
    pub iso: String,
}

impl AttachIsoToServerRequest {
    pub fn new(iso: String) -> AttachIsoToServerRequest {
        AttachIsoToServerRequest {
            iso,
        }
    }
}


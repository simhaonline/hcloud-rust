/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateServerRequestOneOf {
    /// ID or name of Location to create Server in
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl CreateServerRequestOneOf {
    pub fn new() -> CreateServerRequestOneOf {
        CreateServerRequestOneOf {
            location: None,
        }
    }
}



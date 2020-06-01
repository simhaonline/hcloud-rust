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
pub struct ReplaceImageResponse {
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<crate::models::ReplaceImageResponseImage>,
}

impl ReplaceImageResponse {
    pub fn new() -> ReplaceImageResponse {
        ReplaceImageResponse {
            image: None,
        }
    }
}


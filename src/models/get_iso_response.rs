/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetIsoResponse : Response to GET https://api.hetzner.cloud/v1/isos/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIsoResponse {
    #[serde(rename = "iso")]
    pub iso: crate::models::Iso,
}

impl GetIsoResponse {
    /// Response to GET https://api.hetzner.cloud/v1/isos/{id}
    pub fn new(iso: crate::models::Iso) -> GetIsoResponse {
        GetIsoResponse {
            iso,
        }
    }
}



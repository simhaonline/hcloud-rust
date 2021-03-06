/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListServersResponse : Response to GET https://api.hetzner.cloud/v1/servers{?status,sort,name,label_selector}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListServersResponse {
    #[serde(rename = "servers")]
    pub servers: Vec<crate::models::Server>,
}

impl ListServersResponse {
    /// Response to GET https://api.hetzner.cloud/v1/servers{?status,sort,name,label_selector}
    pub fn new(servers: Vec<crate::models::Server>) -> ListServersResponse {
        ListServersResponse {
            servers,
        }
    }
}



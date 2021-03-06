/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateServerResponse : Response to POST https://api.hetzner.cloud/v1/servers



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateServerResponse {
    #[serde(rename = "server")]
    pub server: crate::models::CreateServerResponseServer,
    #[serde(rename = "action")]
    pub action: crate::models::Action,
    #[serde(rename = "next_actions")]
    pub next_actions: Vec<crate::models::Action>,
    /// Root password when no SSH keys have been specified
    #[serde(rename = "root_password")]
    pub root_password: Option<String>,
}

impl CreateServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers
    pub fn new(server: crate::models::CreateServerResponseServer, action: crate::models::Action, next_actions: Vec<crate::models::Action>, root_password: Option<String>) -> CreateServerResponse {
        CreateServerResponse {
            server,
            action,
            next_actions,
            root_password,
        }
    }
}



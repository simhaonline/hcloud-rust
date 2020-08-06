/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Http : Configuration for HTTP and HTTPS services



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Http {
    /// Name of the cookie used for sticky sessions
    #[serde(rename = "cookie_name")]
    pub cookie_name: String,
    /// Lifetime of the cookie used for sticky sessions
    #[serde(rename = "cookie_lifetime")]
    pub cookie_lifetime: i32,
    /// IDs of the Certificates to use for TLS/SSL termination by the Load Balancer; empty for TLS/SSL passthrough or if `protocol` is \"http\"
    #[serde(rename = "certificates", skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<serde_json::Value>>,
    /// Redirect HTTP requests to HTTPS. Only available if protocol is \"https\". Default `false`
    #[serde(rename = "redirect_http", skip_serializing_if = "Option::is_none")]
    pub redirect_http: Option<bool>,
    /// Use sticky sessions. Only available if protocol is \"http\" or \"https\". Default `false`
    #[serde(rename = "sticky_sessions", skip_serializing_if = "Option::is_none")]
    pub sticky_sessions: Option<bool>,
}

impl Http {
    /// Configuration for HTTP and HTTPS services
    pub fn new(cookie_name: String, cookie_lifetime: i32) -> Http {
        Http {
            cookie_name,
            cookie_lifetime,
            certificates: None,
            redirect_http: None,
            sticky_sessions: None,
        }
    }
}


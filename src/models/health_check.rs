/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HealthCheck : Configuration for health checks



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Type of the health check
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    /// Port the health check will be performed on
    #[serde(rename = "port")]
    pub port: i32,
    /// Time interval in seconds health checks are performed
    #[serde(rename = "interval")]
    pub interval: i32,
    /// Time in seconds after an attempt is considered a timeout
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Unsuccessful retries needed until a target is considered unhealthy
    #[serde(rename = "retries")]
    pub retries: i32,
    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<crate::models::HealthCheckHttp>,
}

impl HealthCheck {
    /// Configuration for health checks
    pub fn new(protocol: Protocol, port: i32, interval: i32, timeout: i32, retries: i32) -> HealthCheck {
        HealthCheck {
            protocol,
            port,
            interval,
            timeout,
            retries,
            http: None,
        }
    }
}

/// Type of the health check
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "tcp")]
    Tcp,
}

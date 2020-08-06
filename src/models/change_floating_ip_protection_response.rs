/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeFloatingIpProtectionResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFloatingIpProtectionResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl ChangeFloatingIpProtectionResponse {
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_protection
    pub fn new(action: crate::models::Action) -> ChangeFloatingIpProtectionResponse {
        ChangeFloatingIpProtectionResponse {
            action,
        }
    }
}



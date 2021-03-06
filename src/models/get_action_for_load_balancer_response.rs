/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForLoadBalancerResponse : Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/actions/{action_id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActionForLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl GetActionForLoadBalancerResponse {
    /// Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForLoadBalancerResponse {
        GetActionForLoadBalancerResponse {
            action,
        }
    }
}



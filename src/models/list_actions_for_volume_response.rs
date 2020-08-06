/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListActionsForVolumeResponse : Response to GET https://api.hetzner.cloud/v1/volumes/{id}/actions{?status,sort}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListActionsForVolumeResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
}

impl ListActionsForVolumeResponse {
    /// Response to GET https://api.hetzner.cloud/v1/volumes/{id}/actions{?status,sort}
    pub fn new(actions: Vec<crate::models::Action>) -> ListActionsForVolumeResponse {
        ListActionsForVolumeResponse {
            actions,
        }
    }
}



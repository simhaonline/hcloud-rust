/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;

use std::option::Option;

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `get_iso`
#[derive(Clone, Debug, Default)]
pub struct GetIsoParams {
    /// ID of the ISO
    pub id: String
}

/// struct for passing parameters to the method `list_isos`
#[derive(Clone, Debug, Default)]
pub struct ListIsosParams {
    /// Can be used to filter ISOs by their name. The response will only contain the ISO matching the specified name.
    pub name: Option<String>
}


/// struct for typed errors of method `get_iso`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIsoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_isos`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIsosError {
    UnknownValue(serde_json::Value),
}


    pub async fn get_iso(configuration: &configuration::Configuration, params: GetIsoParams) -> Result<crate::models::GetIsoResponse, Error<GetIsoError>> {
        // unbox the parameters
        let id = params.id;

        let client = &configuration.client;

        let uri_str = format!("{}/isos/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetIsoError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn list_isos(configuration: &configuration::Configuration, params: ListIsosParams) -> Result<crate::models::ListIsosResponse, Error<ListIsosError>> {
        // unbox the parameters
        let name = params.name;

        let client = &configuration.client;

        let uri_str = format!("{}/isos", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = name {
            req_builder = req_builder.query(&[("name", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ListIsosError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }


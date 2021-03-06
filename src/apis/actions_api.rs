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

/// struct for passing parameters to the method `get_action`
#[derive(Clone, Debug, Default)]
pub struct GetActionParams {
    /// ID of the Action
    pub id: String
}

/// struct for passing parameters to the method `list_all_actions`
#[derive(Clone, Debug, Default)]
pub struct ListAllActionsParams {
    /// Can be used multiple times, the response will contain only Actions with specified statuses Choices: running success error
    pub status: Option<String>,
    /// Can be used multiple times Choices: id id:asc id:desc command command:asc command:desc status status:asc status:desc progress progress:asc progress:desc started started:asc started:desc finished finished:asc finished:desc
    pub sort: Option<String>
}


/// struct for typed errors of method `get_action`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_all_actions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAllActionsError {
    UnknownValue(serde_json::Value),
}


    pub async fn get_action(configuration: &configuration::Configuration, params: GetActionParams) -> Result<crate::models::GetActionResponse, Error<GetActionError>> {
        // unbox the parameters
        let id = params.id;

        let client = &configuration.client;

        let uri_str = format!("{}/actions/{id}", configuration.base_path, id=crate::apis::urlencode(id));
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
            let entity: Option<GetActionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn list_all_actions(configuration: &configuration::Configuration, params: ListAllActionsParams) -> Result<crate::models::ListAllActionsResponse, Error<ListAllActionsError>> {
        // unbox the parameters
        let status = params.status;
        let sort = params.sort;

        let client = &configuration.client;

        let uri_str = format!("{}/actions", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = status {
            req_builder = req_builder.query(&[("status", &s.to_string())]);
        }
        if let Some(ref s) = sort {
            req_builder = req_builder.query(&[("sort", &s.to_string())]);
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
            let entity: Option<ListAllActionsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }


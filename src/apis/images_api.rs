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

/// struct for passing parameters to the method `change_image_protection`
#[derive(Clone, Debug, Default)]
pub struct ChangeImageProtectionParams {
    /// ID of the Image
    pub id: String,
    pub change_image_protection_request: Option<crate::models::ChangeImageProtectionRequest>
}

/// struct for passing parameters to the method `delete_image`
#[derive(Clone, Debug, Default)]
pub struct DeleteImageParams {
    /// ID of the Image
    pub id: String
}

/// struct for passing parameters to the method `get_action_for_image`
#[derive(Clone, Debug, Default)]
pub struct GetActionForImageParams {
    /// ID of the Image
    pub id: String,
    /// ID of the Action
    pub action_id: String
}

/// struct for passing parameters to the method `get_image`
#[derive(Clone, Debug, Default)]
pub struct GetImageParams {
    /// ID of the Image
    pub id: String
}

/// struct for passing parameters to the method `list_actions_for_image`
#[derive(Clone, Debug, Default)]
pub struct ListActionsForImageParams {
    /// ID of the Image
    pub id: String,
    /// Can be used multiple times, the response will contain only Actions with specified statuses Choices: running success error
    pub status: Option<String>,
    /// Can be used multiple times Choices: id id:asc id:desc command command:asc command:desc status status:asc status:desc progress progress:asc progress:desc started started:asc started:desc finished finished:asc finished:desc
    pub sort: Option<String>
}

/// struct for passing parameters to the method `list_images`
#[derive(Clone, Debug, Default)]
pub struct ListImagesParams {
    /// Can be used multiple times. Choices: id id:asc id:desc name name:asc name:desc created created:asc created:desc
    pub sort: Option<String>,
    /// Can be used multiple times. Choices: system snapshot backup
    pub _type: Option<String>,
    /// Can be used multiple times. The response will only contain Images matching the status. Choices: available creating
    pub status: Option<String>,
    /// Can be used multiple times. Server ID linked to the Image. Only available for Images of type backup
    pub bound_to: Option<String>,
    /// Can be used to filter Images by their name. The response will only contain the Image matching the specified name.
    pub name: Option<String>,
    /// Can be used to filter Images by labels. The response will only contain Images matching the label selector.
    pub label_selector: Option<String>
}

/// struct for passing parameters to the method `replace_image`
#[derive(Clone, Debug, Default)]
pub struct ReplaceImageParams {
    /// ID of the Image to be updated
    pub id: String,
    pub replace_image_request: Option<crate::models::ReplaceImageRequest>
}


/// struct for typed errors of method `change_image_protection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeImageProtectionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_image`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteImageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_action_for_image`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionForImageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_image`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_actions_for_image`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionsForImageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_images`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListImagesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_image`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceImageError {
    UnknownValue(serde_json::Value),
}


    pub async fn change_image_protection(configuration: &configuration::Configuration, params: ChangeImageProtectionParams) -> Result<crate::models::ChangeImageProtectionResponse, Error<ChangeImageProtectionError>> {
        // unbox the parameters
        let id = params.id;
        let change_image_protection_request = params.change_image_protection_request;

        let client = &configuration.client;

        let uri_str = format!("{}/images/{id}/actions/change_protection", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&change_image_protection_request);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ChangeImageProtectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn delete_image(configuration: &configuration::Configuration, params: DeleteImageParams) -> Result<(), Error<DeleteImageError>> {
        // unbox the parameters
        let id = params.id;

        let client = &configuration.client;

        let uri_str = format!("{}/images/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.delete(uri_str.as_str());

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
            Ok(())
        } else {
            let entity: Option<DeleteImageError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_action_for_image(configuration: &configuration::Configuration, params: GetActionForImageParams) -> Result<crate::models::GetActionForImageResponse, Error<GetActionForImageError>> {
        // unbox the parameters
        let id = params.id;
        let action_id = params.action_id;

        let client = &configuration.client;

        let uri_str = format!("{}/images/{id}/actions/{action_id}", configuration.base_path, id=crate::apis::urlencode(id), action_id=crate::apis::urlencode(action_id));
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
            let entity: Option<GetActionForImageError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_image(configuration: &configuration::Configuration, params: GetImageParams) -> Result<crate::models::GetImageResponse, Error<GetImageError>> {
        // unbox the parameters
        let id = params.id;

        let client = &configuration.client;

        let uri_str = format!("{}/images/{id}", configuration.base_path, id=crate::apis::urlencode(id));
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
            let entity: Option<GetImageError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn list_actions_for_image(configuration: &configuration::Configuration, params: ListActionsForImageParams) -> Result<crate::models::ListActionsForImageResponse, Error<ListActionsForImageError>> {
        // unbox the parameters
        let id = params.id;
        let status = params.status;
        let sort = params.sort;

        let client = &configuration.client;

        let uri_str = format!("{}/images/{id}/actions", configuration.base_path, id=crate::apis::urlencode(id));
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
            let entity: Option<ListActionsForImageError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn list_images(configuration: &configuration::Configuration, params: ListImagesParams) -> Result<crate::models::ListImagesResponse, Error<ListImagesError>> {
        // unbox the parameters
        let sort = params.sort;
        let _type = params._type;
        let status = params.status;
        let bound_to = params.bound_to;
        let name = params.name;
        let label_selector = params.label_selector;

        let client = &configuration.client;

        let uri_str = format!("{}/images", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = sort {
            req_builder = req_builder.query(&[("sort", &s.to_string())]);
        }
        if let Some(ref s) = _type {
            req_builder = req_builder.query(&[("type", &s.to_string())]);
        }
        if let Some(ref s) = status {
            req_builder = req_builder.query(&[("status", &s.to_string())]);
        }
        if let Some(ref s) = bound_to {
            req_builder = req_builder.query(&[("bound_to", &s.to_string())]);
        }
        if let Some(ref s) = name {
            req_builder = req_builder.query(&[("name", &s.to_string())]);
        }
        if let Some(ref s) = label_selector {
            req_builder = req_builder.query(&[("label_selector", &s.to_string())]);
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
            let entity: Option<ListImagesError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn replace_image(configuration: &configuration::Configuration, params: ReplaceImageParams) -> Result<crate::models::ReplaceImageResponse, Error<ReplaceImageError>> {
        // unbox the parameters
        let id = params.id;
        let replace_image_request = params.replace_image_request;

        let client = &configuration.client;

        let uri_str = format!("{}/images/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&replace_image_request);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ReplaceImageError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }


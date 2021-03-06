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

/// struct for passing parameters to the method `add_route_to_network`
#[derive(Clone, Debug, Default)]
pub struct AddRouteToNetworkParams {
    /// ID of the Network
    pub id: String,
    pub body: Option<crate::models::Route>
}

/// struct for passing parameters to the method `add_subnet_to_network`
#[derive(Clone, Debug, Default)]
pub struct AddSubnetToNetworkParams {
    /// ID of the Network
    pub id: String,
    pub body: Option<crate::models::Subnet>
}

/// struct for passing parameters to the method `change_ip_range_of_network`
#[derive(Clone, Debug, Default)]
pub struct ChangeIpRangeOfNetworkParams {
    /// ID of the Network
    pub id: String,
    pub change_ip_range_of_network_request: Option<crate::models::ChangeIpRangeOfNetworkRequest>
}

/// struct for passing parameters to the method `change_network_protection`
#[derive(Clone, Debug, Default)]
pub struct ChangeNetworkProtectionParams {
    /// ID of the Network
    pub id: String,
    pub change_network_protection_request: Option<crate::models::ChangeNetworkProtectionRequest>
}

/// struct for passing parameters to the method `create_network`
#[derive(Clone, Debug, Default)]
pub struct CreateNetworkParams {
    pub create_network_request: Option<crate::models::CreateNetworkRequest>
}

/// struct for passing parameters to the method `delete_network`
#[derive(Clone, Debug, Default)]
pub struct DeleteNetworkParams {
    /// ID of the network
    pub id: String
}

/// struct for passing parameters to the method `delete_route_from_network`
#[derive(Clone, Debug, Default)]
pub struct DeleteRouteFromNetworkParams {
    /// ID of the Network
    pub id: String,
    pub body: Option<crate::models::Route>
}

/// struct for passing parameters to the method `delete_subnet_from_network`
#[derive(Clone, Debug, Default)]
pub struct DeleteSubnetFromNetworkParams {
    /// ID of the Network
    pub id: String,
    pub delete_subnet_from_network_request: Option<crate::models::DeleteSubnetFromNetworkRequest>
}

/// struct for passing parameters to the method `get_action_for_network`
#[derive(Clone, Debug, Default)]
pub struct GetActionForNetworkParams {
    /// ID of the Network
    pub id: String,
    /// ID of the Action
    pub action_id: String
}

/// struct for passing parameters to the method `get_network`
#[derive(Clone, Debug, Default)]
pub struct GetNetworkParams {
    /// ID of the network
    pub id: String
}

/// struct for passing parameters to the method `list_actions_for_network`
#[derive(Clone, Debug, Default)]
pub struct ListActionsForNetworkParams {
    /// ID of the Network
    pub id: String,
    /// Can be used multiple times, the response will contain only Actions with specified statuses Choices: running success error
    pub status: Option<String>,
    /// Can be used multiple times Choices: id id:asc id:desc command command:asc command:desc status status:asc status:desc progress progress:asc progress:desc started started:asc started:desc finished finished:asc finished:desc
    pub sort: Option<String>
}

/// struct for passing parameters to the method `list_networks`
#[derive(Clone, Debug, Default)]
pub struct ListNetworksParams {
    /// Can be used to filter networks by their name. The response will only contain the networks matching the specified name.
    pub name: Option<String>,
    /// Can be used to filter networks by labels. The response will only contain networks with a matching label selector pattern.
    pub label_selector: Option<String>
}

/// struct for passing parameters to the method `replace_network`
#[derive(Clone, Debug, Default)]
pub struct ReplaceNetworkParams {
    /// ID of the network to update
    pub id: String,
    pub replace_network_request: Option<crate::models::ReplaceNetworkRequest>
}


/// struct for typed errors of method `add_route_to_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRouteToNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `add_subnet_to_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSubnetToNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `change_ip_range_of_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeIpRangeOfNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `change_network_protection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeNetworkProtectionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_route_from_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRouteFromNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_subnet_from_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSubnetFromNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_action_for_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionForNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_actions_for_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionsForNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_networks`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNetworksError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_network`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceNetworkError {
    UnknownValue(serde_json::Value),
}


    pub async fn add_route_to_network(configuration: &configuration::Configuration, params: AddRouteToNetworkParams) -> Result<crate::models::AddRouteToNetworkResponse, Error<AddRouteToNetworkError>> {
        // unbox the parameters
        let id = params.id;
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}/actions/add_route", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<AddRouteToNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn add_subnet_to_network(configuration: &configuration::Configuration, params: AddSubnetToNetworkParams) -> Result<crate::models::AddSubnetToNetworkResponse, Error<AddSubnetToNetworkError>> {
        // unbox the parameters
        let id = params.id;
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}/actions/add_subnet", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<AddSubnetToNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn change_ip_range_of_network(configuration: &configuration::Configuration, params: ChangeIpRangeOfNetworkParams) -> Result<crate::models::ChangeIpRangeOfNetworkResponse, Error<ChangeIpRangeOfNetworkError>> {
        // unbox the parameters
        let id = params.id;
        let change_ip_range_of_network_request = params.change_ip_range_of_network_request;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}/actions/change_ip_range", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&change_ip_range_of_network_request);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ChangeIpRangeOfNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn change_network_protection(configuration: &configuration::Configuration, params: ChangeNetworkProtectionParams) -> Result<crate::models::ChangeNetworkProtectionResponse, Error<ChangeNetworkProtectionError>> {
        // unbox the parameters
        let id = params.id;
        let change_network_protection_request = params.change_network_protection_request;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}/actions/change_protection", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&change_network_protection_request);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ChangeNetworkProtectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn create_network(configuration: &configuration::Configuration, params: CreateNetworkParams) -> Result<crate::models::CreateNetworkResponse, Error<CreateNetworkError>> {
        // unbox the parameters
        let create_network_request = params.create_network_request;

        let client = &configuration.client;

        let uri_str = format!("{}/networks", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&create_network_request);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<CreateNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn delete_network(configuration: &configuration::Configuration, params: DeleteNetworkParams) -> Result<(), Error<DeleteNetworkError>> {
        // unbox the parameters
        let id = params.id;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}", configuration.base_path, id=crate::apis::urlencode(id));
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
            let entity: Option<DeleteNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn delete_route_from_network(configuration: &configuration::Configuration, params: DeleteRouteFromNetworkParams) -> Result<crate::models::DeleteRouteFromNetworkResponse, Error<DeleteRouteFromNetworkError>> {
        // unbox the parameters
        let id = params.id;
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}/actions/delete_route", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<DeleteRouteFromNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn delete_subnet_from_network(configuration: &configuration::Configuration, params: DeleteSubnetFromNetworkParams) -> Result<crate::models::DeleteSubnetFromNetworkResponse, Error<DeleteSubnetFromNetworkError>> {
        // unbox the parameters
        let id = params.id;
        let delete_subnet_from_network_request = params.delete_subnet_from_network_request;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}/actions/delete_subnet", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&delete_subnet_from_network_request);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<DeleteSubnetFromNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_action_for_network(configuration: &configuration::Configuration, params: GetActionForNetworkParams) -> Result<crate::models::GetActionForNetworkResponse, Error<GetActionForNetworkError>> {
        // unbox the parameters
        let id = params.id;
        let action_id = params.action_id;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}/actions/{action_id}", configuration.base_path, id=crate::apis::urlencode(id), action_id=crate::apis::urlencode(action_id));
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
            let entity: Option<GetActionForNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_network(configuration: &configuration::Configuration, params: GetNetworkParams) -> Result<crate::models::GetNetworkResponse, Error<GetNetworkError>> {
        // unbox the parameters
        let id = params.id;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}", configuration.base_path, id=crate::apis::urlencode(id));
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
            let entity: Option<GetNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn list_actions_for_network(configuration: &configuration::Configuration, params: ListActionsForNetworkParams) -> Result<crate::models::ListActionsForNetworkResponse, Error<ListActionsForNetworkError>> {
        // unbox the parameters
        let id = params.id;
        let status = params.status;
        let sort = params.sort;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}/actions", configuration.base_path, id=crate::apis::urlencode(id));
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
            let entity: Option<ListActionsForNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn list_networks(configuration: &configuration::Configuration, params: ListNetworksParams) -> Result<crate::models::ListNetworksResponse, Error<ListNetworksError>> {
        // unbox the parameters
        let name = params.name;
        let label_selector = params.label_selector;

        let client = &configuration.client;

        let uri_str = format!("{}/networks", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

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
            let entity: Option<ListNetworksError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn replace_network(configuration: &configuration::Configuration, params: ReplaceNetworkParams) -> Result<crate::models::ReplaceNetworkResponse, Error<ReplaceNetworkError>> {
        // unbox the parameters
        let id = params.id;
        let replace_network_request = params.replace_network_request;

        let client = &configuration.client;

        let uri_str = format!("{}/networks/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&replace_network_request);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ReplaceNetworkError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }


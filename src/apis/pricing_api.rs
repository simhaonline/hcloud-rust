/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct PricingApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl PricingApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> PricingApiClient {
        PricingApiClient {
            configuration,
        }
    }
}

pub trait PricingApi {
    fn list_prices(&self, ) -> Result<crate::models::ListPricesResponse, Error>;
}

impl PricingApi for PricingApiClient {
    fn list_prices(&self, ) -> Result<crate::models::ListPricesResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/pricing", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
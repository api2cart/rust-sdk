/*
 * API2Cart OpenAPI
 *
 * API2Cart
 *
 * The version of the OpenAPI document: 1.1
 * Contact: contact@api2cart.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`return_action_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReturnActionListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`return_count`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReturnCountError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`return_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReturnInfoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`return_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReturnListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`return_reason_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReturnReasonListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`return_status_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReturnStatusListError {
    UnknownValue(serde_json::Value),
}


/// Retrieve list of return actions
pub async fn return_action_list(configuration: &configuration::Configuration, ) -> Result<models::ReturnActionList200Response, Error<ReturnActionListError>> {

    let uri_str = format!("{}/return.action.list.json", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-store-key", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ReturnActionListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Count returns in store
pub async fn return_count(configuration: &configuration::Configuration, order_ids: Option<&str>, customer_id: Option<&str>, store_id: Option<&str>, status: Option<&str>, return_type: Option<&str>, created_from: Option<&str>, created_to: Option<&str>, modified_from: Option<&str>, modified_to: Option<&str>, report_request_id: Option<&str>, disable_report_cache: Option<bool>) -> Result<models::ReturnCount200Response, Error<ReturnCountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_order_ids = order_ids;
    let p_customer_id = customer_id;
    let p_store_id = store_id;
    let p_status = status;
    let p_return_type = return_type;
    let p_created_from = created_from;
    let p_created_to = created_to;
    let p_modified_from = modified_from;
    let p_modified_to = modified_to;
    let p_report_request_id = report_request_id;
    let p_disable_report_cache = disable_report_cache;

    let uri_str = format!("{}/return.count.json", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_order_ids {
        req_builder = req_builder.query(&[("order_ids", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_customer_id {
        req_builder = req_builder.query(&[("customer_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_store_id {
        req_builder = req_builder.query(&[("store_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_status {
        req_builder = req_builder.query(&[("status", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_return_type {
        req_builder = req_builder.query(&[("return_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_from {
        req_builder = req_builder.query(&[("created_from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_to {
        req_builder = req_builder.query(&[("created_to", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_modified_from {
        req_builder = req_builder.query(&[("modified_from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_modified_to {
        req_builder = req_builder.query(&[("modified_to", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_report_request_id {
        req_builder = req_builder.query(&[("report_request_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_disable_report_cache {
        req_builder = req_builder.query(&[("disable_report_cache", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-store-key", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ReturnCountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve return information.
pub async fn return_info(configuration: &configuration::Configuration, id: &str, order_id: Option<&str>, store_id: Option<&str>, params: Option<&str>, exclude: Option<&str>, response_fields: Option<&str>) -> Result<models::ReturnInfo200Response, Error<ReturnInfoError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_order_id = order_id;
    let p_store_id = store_id;
    let p_params = params;
    let p_exclude = exclude;
    let p_response_fields = response_fields;

    let uri_str = format!("{}/return.info.json", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref param_value) = p_order_id {
        req_builder = req_builder.query(&[("order_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_store_id {
        req_builder = req_builder.query(&[("store_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_params {
        req_builder = req_builder.query(&[("params", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_exclude {
        req_builder = req_builder.query(&[("exclude", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_response_fields {
        req_builder = req_builder.query(&[("response_fields", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-store-key", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ReturnInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get list of return requests from store.
pub async fn return_list(configuration: &configuration::Configuration, start: Option<i32>, count: Option<i32>, page_cursor: Option<&str>, params: Option<&str>, exclude: Option<&str>, response_fields: Option<&str>, order_id: Option<&str>, order_ids: Option<&str>, customer_id: Option<&str>, store_id: Option<&str>, status: Option<&str>, return_type: Option<&str>, created_from: Option<&str>, created_to: Option<&str>, modified_from: Option<&str>, modified_to: Option<&str>, report_request_id: Option<&str>, disable_report_cache: Option<bool>) -> Result<models::ModelResponseReturnList, Error<ReturnListError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_start = start;
    let p_count = count;
    let p_page_cursor = page_cursor;
    let p_params = params;
    let p_exclude = exclude;
    let p_response_fields = response_fields;
    let p_order_id = order_id;
    let p_order_ids = order_ids;
    let p_customer_id = customer_id;
    let p_store_id = store_id;
    let p_status = status;
    let p_return_type = return_type;
    let p_created_from = created_from;
    let p_created_to = created_to;
    let p_modified_from = modified_from;
    let p_modified_to = modified_to;
    let p_report_request_id = report_request_id;
    let p_disable_report_cache = disable_report_cache;

    let uri_str = format!("{}/return.list.json", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_count {
        req_builder = req_builder.query(&[("count", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_cursor {
        req_builder = req_builder.query(&[("page_cursor", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_params {
        req_builder = req_builder.query(&[("params", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_exclude {
        req_builder = req_builder.query(&[("exclude", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_response_fields {
        req_builder = req_builder.query(&[("response_fields", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_order_id {
        req_builder = req_builder.query(&[("order_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_order_ids {
        req_builder = req_builder.query(&[("order_ids", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_customer_id {
        req_builder = req_builder.query(&[("customer_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_store_id {
        req_builder = req_builder.query(&[("store_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_status {
        req_builder = req_builder.query(&[("status", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_return_type {
        req_builder = req_builder.query(&[("return_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_from {
        req_builder = req_builder.query(&[("created_from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_to {
        req_builder = req_builder.query(&[("created_to", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_modified_from {
        req_builder = req_builder.query(&[("modified_from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_modified_to {
        req_builder = req_builder.query(&[("modified_to", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_report_request_id {
        req_builder = req_builder.query(&[("report_request_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_disable_report_cache {
        req_builder = req_builder.query(&[("disable_report_cache", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-store-key", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ReturnListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve list of return reasons
pub async fn return_reason_list(configuration: &configuration::Configuration, store_id: Option<&str>) -> Result<models::ReturnReasonList200Response, Error<ReturnReasonListError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_store_id = store_id;

    let uri_str = format!("{}/return.reason.list.json", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_store_id {
        req_builder = req_builder.query(&[("store_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-store-key", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ReturnReasonListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve list of statuses
pub async fn return_status_list(configuration: &configuration::Configuration, ) -> Result<models::ReturnStatusList200Response, Error<ReturnStatusListError>> {

    let uri_str = format!("{}/return.status.list.json", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-store-key", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ReturnStatusListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}


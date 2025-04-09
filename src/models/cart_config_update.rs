/*
 * API2Cart OpenAPI
 *
 * API2Cart
 *
 * The version of the OpenAPI document: 1.1
 * Contact: contact@api2cart.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CartConfigUpdate {
    /// This parameter is deprecated for this method. Please, use this parameter in method account.config.update
    #[serde(rename = "db_tables_prefix", skip_serializing_if = "Option::is_none")]
    pub db_tables_prefix: Option<String>,
    /// This parameter sets the list of params to the shopping cart.
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    /// Store Id
    #[serde(rename = "store_id", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// This parameter allows you to set your custom user agent, which will be used in requests to the store. Please use it cautiously, as the store's firewall may block specific values.
    #[serde(rename = "user_agent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl CartConfigUpdate {
    pub fn new() -> CartConfigUpdate {
        CartConfigUpdate {
            db_tables_prefix: None,
            custom_fields: None,
            store_id: None,
            user_agent: None,
        }
    }
}


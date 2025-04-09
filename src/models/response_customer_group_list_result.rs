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
pub struct ResponseCustomerGroupListResult {
    #[serde(rename = "group_count", skip_serializing_if = "Option::is_none")]
    pub group_count: Option<i32>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<models::CustomerGroup>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl ResponseCustomerGroupListResult {
    pub fn new() -> ResponseCustomerGroupListResult {
        ResponseCustomerGroupListResult {
            group_count: None,
            group: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


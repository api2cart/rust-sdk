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
pub struct BasketLiveShippingService {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "callback", skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,
    #[serde(rename = "callback_err_cnt", skip_serializing_if = "Option::is_none")]
    pub callback_err_cnt: Option<i32>,
    #[serde(rename = "enabled_on_store", skip_serializing_if = "Option::is_none")]
    pub enabled_on_store: Option<bool>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl BasketLiveShippingService {
    pub fn new() -> BasketLiveShippingService {
        BasketLiveShippingService {
            id: None,
            name: None,
            callback: None,
            callback_err_cnt: None,
            enabled_on_store: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


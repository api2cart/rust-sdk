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
pub struct ModelResponseReturnList {
    #[serde(rename = "return_code", skip_serializing_if = "Option::is_none")]
    pub return_code: Option<i32>,
    #[serde(rename = "return_message", skip_serializing_if = "Option::is_none")]
    pub return_message: Option<String>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<models::Pagination>>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::ResponseReturnListResult>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl ModelResponseReturnList {
    pub fn new() -> ModelResponseReturnList {
        ModelResponseReturnList {
            return_code: None,
            return_message: None,
            pagination: None,
            result: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


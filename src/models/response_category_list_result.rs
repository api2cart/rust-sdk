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
pub struct ResponseCategoryListResult {
    #[serde(rename = "categories_count", skip_serializing_if = "Option::is_none")]
    pub categories_count: Option<i32>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<models::Category>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl ResponseCategoryListResult {
    pub fn new() -> ResponseCategoryListResult {
        ResponseCategoryListResult {
            categories_count: None,
            category: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


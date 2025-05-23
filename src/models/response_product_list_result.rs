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
pub struct ResponseProductListResult {
    #[serde(rename = "products_count", skip_serializing_if = "Option::is_none")]
    pub products_count: Option<i32>,
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<Vec<models::Product>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl ResponseProductListResult {
    pub fn new() -> ResponseProductListResult {
        ResponseProductListResult {
            products_count: None,
            product: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


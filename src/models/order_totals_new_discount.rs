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
pub struct OrderTotalsNewDiscount {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl OrderTotalsNewDiscount {
    pub fn new() -> OrderTotalsNewDiscount {
        OrderTotalsNewDiscount {
            code: None,
            value: None,
            r#type: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


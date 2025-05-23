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
pub struct OrderStatusRefundItem {
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "variant_id", skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    #[serde(rename = "order_product_id", skip_serializing_if = "Option::is_none")]
    pub order_product_id: Option<String>,
    #[serde(rename = "qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<f64>,
    #[serde(rename = "refund", skip_serializing_if = "Option::is_none")]
    pub refund: Option<f64>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl OrderStatusRefundItem {
    pub fn new() -> OrderStatusRefundItem {
        OrderStatusRefundItem {
            product_id: None,
            variant_id: None,
            order_product_id: None,
            qty: None,
            refund: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


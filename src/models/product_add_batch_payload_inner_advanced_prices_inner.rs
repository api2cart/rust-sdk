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
pub struct ProductAddBatchPayloadInnerAdvancedPricesInner {
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    #[serde(rename = "quantity")]
    pub quantity: f64,
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "expire_time", skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
}

impl ProductAddBatchPayloadInnerAdvancedPricesInner {
    pub fn new(value: f64, quantity: f64) -> ProductAddBatchPayloadInnerAdvancedPricesInner {
        ProductAddBatchPayloadInnerAdvancedPricesInner {
            value,
            group_id: None,
            quantity,
            start_time: None,
            expire_time: None,
        }
    }
}


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
pub struct ResponseOrderShipmentListResult {
    #[serde(rename = "shipment_count", skip_serializing_if = "Option::is_none")]
    pub shipment_count: Option<i32>,
    #[serde(rename = "shipment", skip_serializing_if = "Option::is_none")]
    pub shipment: Option<Vec<models::Shipment>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl ResponseOrderShipmentListResult {
    pub fn new() -> ResponseOrderShipmentListResult {
        ResponseOrderShipmentListResult {
            shipment_count: None,
            shipment: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


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
pub struct Shipment {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "warehouse_id", skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "shipment_provider", skip_serializing_if = "Option::is_none")]
    pub shipment_provider: Option<String>,
    #[serde(rename = "tracking_numbers", skip_serializing_if = "Option::is_none")]
    pub tracking_numbers: Option<Vec<models::ShipmentTrackingNumber>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Box<models::A2CDateTime>>,
    #[serde(rename = "modified_time", skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<Box<models::A2CDateTime>>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::ShipmentItem>>,
    #[serde(rename = "is_shipped", skip_serializing_if = "Option::is_none")]
    pub is_shipped: Option<bool>,
    #[serde(rename = "delivered_at", skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<Box<models::A2CDateTime>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl Shipment {
    pub fn new() -> Shipment {
        Shipment {
            id: None,
            order_id: None,
            name: None,
            warehouse_id: None,
            shipment_provider: None,
            tracking_numbers: None,
            created_at: None,
            modified_time: None,
            items: None,
            is_shipped: None,
            delivered_at: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


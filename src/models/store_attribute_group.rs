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
pub struct StoreAttributeGroup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "attribute_set_id", skip_serializing_if = "Option::is_none")]
    pub attribute_set_id: Option<String>,
    #[serde(rename = "assigned_attribute_ids", skip_serializing_if = "Option::is_none")]
    pub assigned_attribute_ids: Option<Vec<String>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl StoreAttributeGroup {
    pub fn new() -> StoreAttributeGroup {
        StoreAttributeGroup {
            id: None,
            name: None,
            position: None,
            attribute_set_id: None,
            assigned_attribute_ids: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


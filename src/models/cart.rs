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
pub struct Cart {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "db_prefix", skip_serializing_if = "Option::is_none")]
    pub db_prefix: Option<String>,
    #[serde(rename = "stores_info", skip_serializing_if = "Option::is_none")]
    pub stores_info: Option<Vec<models::CartStoreInfo>>,
    #[serde(rename = "warehouses", skip_serializing_if = "Option::is_none")]
    pub warehouses: Option<Vec<models::CartWarehouse>>,
    #[serde(rename = "shipping_zones", skip_serializing_if = "Option::is_none")]
    pub shipping_zones: Option<Vec<models::CartShippingZone>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            name: None,
            url: None,
            version: None,
            db_prefix: None,
            stores_info: None,
            warehouses: None,
            shipping_zones: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


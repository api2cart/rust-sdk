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
pub struct BasketItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "variant_id", skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    #[serde(rename = "sku", skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(rename = "weight_unit", skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<models::BasketItemOption>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl BasketItem {
    pub fn new() -> BasketItem {
        BasketItem {
            id: None,
            parent_id: None,
            product_id: None,
            variant_id: None,
            sku: None,
            name: None,
            price: None,
            tax: None,
            quantity: None,
            weight_unit: None,
            weight: None,
            options: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


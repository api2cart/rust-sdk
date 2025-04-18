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
pub struct AccountCartList200ResponseResultCartsInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "store_key", skip_serializing_if = "Option::is_none")]
    pub store_key: Option<String>,
    #[serde(rename = "cart_id", skip_serializing_if = "Option::is_none")]
    pub cart_id: Option<String>,
    #[serde(rename = "total_calls", skip_serializing_if = "Option::is_none")]
    pub total_calls: Option<String>,
}

impl AccountCartList200ResponseResultCartsInner {
    pub fn new() -> AccountCartList200ResponseResultCartsInner {
        AccountCartList200ResponseResultCartsInner {
            id: None,
            url: None,
            store_key: None,
            cart_id: None,
            total_calls: None,
        }
    }
}


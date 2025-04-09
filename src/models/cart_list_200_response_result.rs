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
pub struct CartList200ResponseResult {
    #[serde(rename = "supported_carts", skip_serializing_if = "Option::is_none")]
    pub supported_carts: Option<Vec<models::CartList200ResponseResultSupportedCartsInner>>,
}

impl CartList200ResponseResult {
    pub fn new() -> CartList200ResponseResult {
        CartList200ResponseResult {
            supported_carts: None,
        }
    }
}


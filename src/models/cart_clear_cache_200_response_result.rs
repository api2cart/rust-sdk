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
pub struct CartClearCache200ResponseResult {
    #[serde(rename = "cache_cleared", skip_serializing_if = "Option::is_none")]
    pub cache_cleared: Option<String>,
}

impl CartClearCache200ResponseResult {
    pub fn new() -> CartClearCache200ResponseResult {
        CartClearCache200ResponseResult {
            cache_cleared: None,
        }
    }
}


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
pub struct ProductImageUpdate200ResponseResult {
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl ProductImageUpdate200ResponseResult {
    pub fn new() -> ProductImageUpdate200ResponseResult {
        ProductImageUpdate200ResponseResult {
            updated: None,
        }
    }
}


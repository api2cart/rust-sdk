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
pub struct AttributeDelete200ResponseResult {
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
}

impl AttributeDelete200ResponseResult {
    pub fn new() -> AttributeDelete200ResponseResult {
        AttributeDelete200ResponseResult {
            deleted: None,
        }
    }
}


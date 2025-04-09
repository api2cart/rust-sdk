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
pub struct AttributeTypeList200ResponseResult {
    #[serde(rename = "attribute_type", skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<Vec<String>>,
}

impl AttributeTypeList200ResponseResult {
    pub fn new() -> AttributeTypeList200ResponseResult {
        AttributeTypeList200ResponseResult {
            attribute_type: None,
        }
    }
}


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
pub struct ProductImageAdd200ResponseResult {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "image_path", skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
}

impl ProductImageAdd200ResponseResult {
    pub fn new() -> ProductImageAdd200ResponseResult {
        ProductImageAdd200ResponseResult {
            id: None,
            image_path: None,
        }
    }
}


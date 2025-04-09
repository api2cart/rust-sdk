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
pub struct ProductVariantCount200Response {
    #[serde(rename = "return_code", skip_serializing_if = "Option::is_none")]
    pub return_code: Option<i32>,
    #[serde(rename = "return_message", skip_serializing_if = "Option::is_none")]
    pub return_message: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::ProductVariantCount200ResponseResult>>,
}

impl ProductVariantCount200Response {
    pub fn new() -> ProductVariantCount200Response {
        ProductVariantCount200Response {
            return_code: None,
            return_message: None,
            result: None,
        }
    }
}


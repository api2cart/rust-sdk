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
pub struct ProductVariantUpdateOptionsInner {
    #[serde(rename = "option_name", skip_serializing_if = "Option::is_none")]
    pub option_name: Option<String>,
    #[serde(rename = "option_value", skip_serializing_if = "Option::is_none")]
    pub option_value: Option<String>,
}

impl ProductVariantUpdateOptionsInner {
    pub fn new() -> ProductVariantUpdateOptionsInner {
        ProductVariantUpdateOptionsInner {
            option_name: None,
            option_value: None,
        }
    }
}


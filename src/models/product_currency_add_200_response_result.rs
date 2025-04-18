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
pub struct ProductCurrencyAdd200ResponseResult {
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
}

impl ProductCurrencyAdd200ResponseResult {
    pub fn new() -> ProductCurrencyAdd200ResponseResult {
        ProductCurrencyAdd200ResponseResult {
            currency_id: None,
        }
    }
}


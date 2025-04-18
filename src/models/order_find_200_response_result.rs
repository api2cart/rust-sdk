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
pub struct OrderFind200ResponseResult {
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<models::Order>>,
}

impl OrderFind200ResponseResult {
    pub fn new() -> OrderFind200ResponseResult {
        OrderFind200ResponseResult {
            order: None,
        }
    }
}


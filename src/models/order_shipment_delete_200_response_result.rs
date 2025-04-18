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
pub struct OrderShipmentDelete200ResponseResult {
    #[serde(rename = "deleted_items", skip_serializing_if = "Option::is_none")]
    pub deleted_items: Option<i32>,
}

impl OrderShipmentDelete200ResponseResult {
    pub fn new() -> OrderShipmentDelete200ResponseResult {
        OrderShipmentDelete200ResponseResult {
            deleted_items: None,
        }
    }
}


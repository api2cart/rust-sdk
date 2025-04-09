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
pub struct ResponseCartCouponListResult {
    #[serde(rename = "coupon_count", skip_serializing_if = "Option::is_none")]
    pub coupon_count: Option<i32>,
    #[serde(rename = "coupon", skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Vec<models::Coupon>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl ResponseCartCouponListResult {
    pub fn new() -> ResponseCartCouponListResult {
        ResponseCartCouponListResult {
            coupon_count: None,
            coupon: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


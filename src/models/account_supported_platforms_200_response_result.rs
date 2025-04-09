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
pub struct AccountSupportedPlatforms200ResponseResult {
    #[serde(rename = "supported_platforms", skip_serializing_if = "Option::is_none")]
    pub supported_platforms: Option<Vec<models::AccountSupportedPlatforms200ResponseResultSupportedPlatformsInner>>,
}

impl AccountSupportedPlatforms200ResponseResult {
    pub fn new() -> AccountSupportedPlatforms200ResponseResult {
        AccountSupportedPlatforms200ResponseResult {
            supported_platforms: None,
        }
    }
}


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
pub struct TaxClassZipCodes {
    #[serde(rename = "is_range", skip_serializing_if = "Option::is_none")]
    pub is_range: Option<bool>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Vec<String>>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<models::TaxClassZipCodesRange>>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl TaxClassZipCodes {
    pub fn new() -> TaxClassZipCodes {
        TaxClassZipCodes {
            is_range: None,
            range: None,
            fields: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


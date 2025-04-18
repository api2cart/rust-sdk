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
pub struct ProductVariantAddBatch {
    #[serde(rename = "clear_cache", skip_serializing_if = "Option::is_none")]
    pub clear_cache: Option<bool>,
    #[serde(rename = "reindex", skip_serializing_if = "Option::is_none")]
    pub reindex: Option<bool>,
    /// Contains an array of product variants objects. The list of properties may vary depending on the specific platform.
    #[serde(rename = "payload")]
    pub payload: Vec<models::ProductVariantAddBatchPayloadInner>,
}

impl ProductVariantAddBatch {
    pub fn new(payload: Vec<models::ProductVariantAddBatchPayloadInner>) -> ProductVariantAddBatch {
        ProductVariantAddBatch {
            clear_cache: None,
            reindex: None,
            payload,
        }
    }
}


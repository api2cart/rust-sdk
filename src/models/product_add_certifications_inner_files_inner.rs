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
pub struct ProductAddCertificationsInnerFilesInner {
    /// File URL
    #[serde(rename = "url")]
    pub url: String,
}

impl ProductAddCertificationsInnerFilesInner {
    pub fn new(url: String) -> ProductAddCertificationsInnerFilesInner {
        ProductAddCertificationsInnerFilesInner {
            url,
        }
    }
}


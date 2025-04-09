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

/// ProductAddSizeChart : A size chart for the product. Only one property is supported.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductAddSizeChart {
    /// Defines a pre-generated size chart template
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Defines a size chart image URL
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ProductAddSizeChart {
    /// A size chart for the product. Only one property is supported.
    pub fn new() -> ProductAddSizeChart {
        ProductAddSizeChart {
            id: None,
            url: None,
        }
    }
}


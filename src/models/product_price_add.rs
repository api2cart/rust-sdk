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
pub struct ProductPriceAdd {
    /// Defines the product to which the price has to be added
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Defines product's group prices
    #[serde(rename = "group_prices", skip_serializing_if = "Option::is_none")]
    pub group_prices: Option<Vec<models::ProductAddGroupPricesInner>>,
    /// Store Id
    #[serde(rename = "store_id", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
}

impl ProductPriceAdd {
    pub fn new() -> ProductPriceAdd {
        ProductPriceAdd {
            product_id: None,
            group_prices: None,
            store_id: None,
        }
    }
}


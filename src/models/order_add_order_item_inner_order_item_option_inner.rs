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
pub struct OrderAddOrderItemInnerOrderItemOptionInner {
    /// Ordered Product Option Name. Where x is order item ID, y is order item option ID
    #[serde(rename = "order_item_option_name", skip_serializing_if = "Option::is_none")]
    pub order_item_option_name: Option<String>,
    /// Ordered product option value Where x is order item ID, y - order item option ID
    #[serde(rename = "order_item_option_value", skip_serializing_if = "Option::is_none")]
    pub order_item_option_value: Option<String>,
    /// Ordered product option price Where x is order item ID, y - order item option ID
    #[serde(rename = "order_item_option_price", skip_serializing_if = "Option::is_none")]
    pub order_item_option_price: Option<f64>,
}

impl OrderAddOrderItemInnerOrderItemOptionInner {
    pub fn new() -> OrderAddOrderItemInnerOrderItemOptionInner {
        OrderAddOrderItemInnerOrderItemOptionInner {
            order_item_option_name: None,
            order_item_option_value: None,
            order_item_option_price: None,
        }
    }
}


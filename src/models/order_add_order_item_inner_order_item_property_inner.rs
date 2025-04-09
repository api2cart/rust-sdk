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
pub struct OrderAddOrderItemInnerOrderItemPropertyInner {
    /// Ordered product property name. Where x is order item ID, y is order item property ID
    #[serde(rename = "order_item_property_name", skip_serializing_if = "Option::is_none")]
    pub order_item_property_name: Option<String>,
    /// Ordered product property value. Where x is order item ID, y - order item property ID
    #[serde(rename = "order_item_property_value", skip_serializing_if = "Option::is_none")]
    pub order_item_property_value: Option<String>,
}

impl OrderAddOrderItemInnerOrderItemPropertyInner {
    pub fn new() -> OrderAddOrderItemInnerOrderItemPropertyInner {
        OrderAddOrderItemInnerOrderItemPropertyInner {
            order_item_property_name: None,
            order_item_property_value: None,
        }
    }
}


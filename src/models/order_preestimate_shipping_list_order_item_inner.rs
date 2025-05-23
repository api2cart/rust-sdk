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
pub struct OrderPreestimateShippingListOrderItemInner {
    /// Defines orders specified by order item id
    #[serde(rename = "order_item_id")]
    pub order_item_id: String,
    /// Defines orders specified by order item model
    #[serde(rename = "order_item_model", skip_serializing_if = "Option::is_none")]
    pub order_item_model: Option<String>,
    /// Defines orders specified by order item quantity
    #[serde(rename = "order_item_quantity")]
    pub order_item_quantity: i32,
    /// Defines orders specified by order item weight
    #[serde(rename = "order_item_weight", skip_serializing_if = "Option::is_none")]
    pub order_item_weight: Option<f64>,
    /// Ordered product variant. Where x is order item ID
    #[serde(rename = "order_item_variant_id", skip_serializing_if = "Option::is_none")]
    pub order_item_variant_id: Option<String>,
    #[serde(rename = "order_item_option", skip_serializing_if = "Option::is_none")]
    pub order_item_option: Option<Vec<models::OrderPreestimateShippingListOrderItemInnerOrderItemOptionInner>>,
}

impl OrderPreestimateShippingListOrderItemInner {
    pub fn new(order_item_id: String, order_item_quantity: i32) -> OrderPreestimateShippingListOrderItemInner {
        OrderPreestimateShippingListOrderItemInner {
            order_item_id,
            order_item_model: None,
            order_item_quantity,
            order_item_weight: None,
            order_item_variant_id: None,
            order_item_option: None,
        }
    }
}


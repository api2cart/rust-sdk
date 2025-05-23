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
pub struct OrderReturnAdd {
    /// Defines the order id
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Store Id
    #[serde(rename = "store_id", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// Defines return request status
    #[serde(rename = "return_status_id")]
    pub return_status_id: String,
    /// Defines return request action
    #[serde(rename = "return_action_id")]
    pub return_action_id: String,
    /// Defines return request reason
    #[serde(rename = "return_reason_id")]
    pub return_reason_id: String,
    /// Defines return request reason
    #[serde(rename = "return_reason", skip_serializing_if = "Option::is_none")]
    pub return_reason: Option<String>,
    /// Boolean, whether or not to add the line items back to the store inventory.
    #[serde(rename = "item_restock", skip_serializing_if = "Option::is_none")]
    pub item_restock: Option<bool>,
    /// Specifies staff note
    #[serde(rename = "staff_note", skip_serializing_if = "Option::is_none")]
    pub staff_note: Option<String>,
    /// Specifies return comment
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Send notifications to customer after order was created
    #[serde(rename = "send_notifications", skip_serializing_if = "Option::is_none")]
    pub send_notifications: Option<bool>,
    /// Defines return reject reason
    #[serde(rename = "reject_reason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(rename = "order_products")]
    pub order_products: Vec<models::OrderReturnAddOrderProductsInner>,
}

impl OrderReturnAdd {
    pub fn new(return_status_id: String, return_action_id: String, return_reason_id: String, order_products: Vec<models::OrderReturnAddOrderProductsInner>) -> OrderReturnAdd {
        OrderReturnAdd {
            order_id: None,
            store_id: None,
            return_status_id,
            return_action_id,
            return_reason_id,
            return_reason: None,
            item_restock: None,
            staff_note: None,
            comment: None,
            send_notifications: None,
            reject_reason: None,
            order_products,
        }
    }
}


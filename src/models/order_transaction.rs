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
pub struct OrderTransaction {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "reference_number", skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<Box<models::A2CDateTime>>,
    #[serde(rename = "settlement_currency", skip_serializing_if = "Option::is_none")]
    pub settlement_currency: Option<String>,
    #[serde(rename = "settlement_amount", skip_serializing_if = "Option::is_none")]
    pub settlement_amount: Option<f64>,
    #[serde(rename = "settlement_created_time", skip_serializing_if = "Option::is_none")]
    pub settlement_created_time: Option<Box<models::A2CDateTime>>,
    #[serde(rename = "card_brand", skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    #[serde(rename = "card_bin", skip_serializing_if = "Option::is_none")]
    pub card_bin: Option<String>,
    #[serde(rename = "card_last_four", skip_serializing_if = "Option::is_none")]
    pub card_last_four: Option<String>,
    #[serde(rename = "avs_street_resp_code", skip_serializing_if = "Option::is_none")]
    pub avs_street_resp_code: Option<String>,
    #[serde(rename = "avs_postal_resp_code", skip_serializing_if = "Option::is_none")]
    pub avs_postal_resp_code: Option<String>,
    #[serde(rename = "avs_message", skip_serializing_if = "Option::is_none")]
    pub avs_message: Option<String>,
    #[serde(rename = "cvv_code", skip_serializing_if = "Option::is_none")]
    pub cvv_code: Option<String>,
    #[serde(rename = "cvv_message", skip_serializing_if = "Option::is_none")]
    pub cvv_message: Option<String>,
    #[serde(rename = "is_test_mode", skip_serializing_if = "Option::is_none")]
    pub is_test_mode: Option<bool>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl OrderTransaction {
    pub fn new() -> OrderTransaction {
        OrderTransaction {
            id: None,
            transaction_id: None,
            order_id: None,
            parent_id: None,
            description: None,
            status: None,
            gateway: None,
            reference_number: None,
            currency: None,
            amount: None,
            created_time: None,
            settlement_currency: None,
            settlement_amount: None,
            settlement_created_time: None,
            card_brand: None,
            card_bin: None,
            card_last_four: None,
            avs_street_resp_code: None,
            avs_postal_resp_code: None,
            avs_message: None,
            cvv_code: None,
            cvv_message: None,
            is_test_mode: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


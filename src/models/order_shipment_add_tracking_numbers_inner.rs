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
pub struct OrderShipmentAddTrackingNumbersInner {
    #[serde(rename = "carrier_id", skip_serializing_if = "Option::is_none")]
    pub carrier_id: Option<String>,
    #[serde(rename = "tracking_number", skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

impl OrderShipmentAddTrackingNumbersInner {
    pub fn new() -> OrderShipmentAddTrackingNumbersInner {
        OrderShipmentAddTrackingNumbersInner {
            carrier_id: None,
            tracking_number: None,
        }
    }
}


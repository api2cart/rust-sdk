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
pub struct OrderFinancialStatusList200ResponseResultOrderFinancialStatusesInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl OrderFinancialStatusList200ResponseResultOrderFinancialStatusesInner {
    pub fn new() -> OrderFinancialStatusList200ResponseResultOrderFinancialStatusesInner {
        OrderFinancialStatusList200ResponseResultOrderFinancialStatusesInner {
            id: None,
            name: None,
        }
    }
}


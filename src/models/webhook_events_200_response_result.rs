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
pub struct WebhookEvents200ResponseResult {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<models::WebhookEvents200ResponseResultEventsInner>>,
}

impl WebhookEvents200ResponseResult {
    pub fn new() -> WebhookEvents200ResponseResult {
        WebhookEvents200ResponseResult {
            events: None,
        }
    }
}


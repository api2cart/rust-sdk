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
pub struct ProductAddSpecificsInnerBookingDetailsAvailabilitiesInnerTimesInner {
    /// The starting time of the of available booking slot in 24 hours format. Required if <code>type=date_time</code>
    #[serde(rename = "from")]
    pub from: String,
    /// The ending time of the of available booking slot in 24 hours format. Required if <code>type=date_time</code>
    #[serde(rename = "to")]
    pub to: String,
}

impl ProductAddSpecificsInnerBookingDetailsAvailabilitiesInnerTimesInner {
    pub fn new(from: String, to: String) -> ProductAddSpecificsInnerBookingDetailsAvailabilitiesInnerTimesInner {
        ProductAddSpecificsInnerBookingDetailsAvailabilitiesInnerTimesInner {
            from,
            to,
        }
    }
}


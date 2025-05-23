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

/// ProductAddSellerProfiles : If the seller is subscribed to \"Business Policies\", use the seller_profiles instead of the shipping_details, payment_methods and return_accepted params.<hr><div style=\"font-style:normal\">Param structure:<div style=\"margin-left: 2%;\"><code style=\"padding:0; background-color:#ffffff;font-size:85%;font-family:monospace;\">seller_profiles[<b>shipping_profile_id</b>] = string</br>seller_profiles[<b>payment_profile_id</b>] = string</br>seller_profiles[<b>return_profile_id</b>] = string</br></code></div></div>
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductAddSellerProfiles {
    #[serde(rename = "shipping_profile_id", skip_serializing_if = "Option::is_none")]
    pub shipping_profile_id: Option<String>,
    #[serde(rename = "payment_profile_id", skip_serializing_if = "Option::is_none")]
    pub payment_profile_id: Option<String>,
    #[serde(rename = "return_profile_id", skip_serializing_if = "Option::is_none")]
    pub return_profile_id: Option<String>,
}

impl ProductAddSellerProfiles {
    /// If the seller is subscribed to \"Business Policies\", use the seller_profiles instead of the shipping_details, payment_methods and return_accepted params.<hr><div style=\"font-style:normal\">Param structure:<div style=\"margin-left: 2%;\"><code style=\"padding:0; background-color:#ffffff;font-size:85%;font-family:monospace;\">seller_profiles[<b>shipping_profile_id</b>] = string</br>seller_profiles[<b>payment_profile_id</b>] = string</br>seller_profiles[<b>return_profile_id</b>] = string</br></code></div></div>
    pub fn new() -> ProductAddSellerProfiles {
        ProductAddSellerProfiles {
            shipping_profile_id: None,
            payment_profile_id: None,
            return_profile_id: None,
        }
    }
}


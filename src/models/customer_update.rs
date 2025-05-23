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
pub struct CustomerUpdate {
    /// Entity id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Customer group_id
    #[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Groups that will be assigned to a customer
    #[serde(rename = "group_ids", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Defines the group where the customer
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Defines customer's email
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Defines customer's phone number
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Defines customer's first name
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Defines customer's last name
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Defines customer's birthday
    #[serde(rename = "birth_day", skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<String>,
    /// Defines whether the newsletter subscription is available for the user
    #[serde(rename = "news_letter_subscription", skip_serializing_if = "Option::is_none")]
    pub news_letter_subscription: Option<bool>,
    /// Defines consents to notifications
    #[serde(rename = "consents", skip_serializing_if = "Option::is_none")]
    pub consents: Option<Vec<models::CustomerAddConsentsInner>>,
    /// Customer tags
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Defines customer's gender
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// The customer note.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Defines customer's status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Store Id
    #[serde(rename = "store_id", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<models::CustomerUpdateAddressInner>>,
}

impl CustomerUpdate {
    pub fn new() -> CustomerUpdate {
        CustomerUpdate {
            id: None,
            group_id: None,
            group_ids: None,
            group: None,
            email: None,
            phone: None,
            first_name: None,
            last_name: None,
            birth_day: None,
            news_letter_subscription: None,
            consents: None,
            tags: None,
            gender: None,
            note: None,
            status: None,
            store_id: None,
            address: None,
        }
    }
}


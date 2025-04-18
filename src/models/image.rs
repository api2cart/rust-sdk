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
pub struct Image {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "http_path", skip_serializing_if = "Option::is_none")]
    pub http_path: Option<String>,
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "mime-type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<Box<models::A2CDateTime>>,
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Box<models::A2CDateTime>>,
    #[serde(rename = "alt", skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(rename = "avail", skip_serializing_if = "Option::is_none")]
    pub avail: Option<bool>,
    #[serde(rename = "sort_order", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "additional_fields", skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<serde_json::Value>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl Image {
    pub fn new() -> Image {
        Image {
            id: None,
            http_path: None,
            file_name: None,
            mime_type: None,
            size: None,
            create_at: None,
            modified_at: None,
            alt: None,
            avail: None,
            sort_order: None,
            r#type: None,
            additional_fields: None,
            custom_fields: None,
        }
    }
}


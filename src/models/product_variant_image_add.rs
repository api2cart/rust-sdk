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
pub struct ProductVariantImageAdd {
    /// Defines product id where the variant image has to be added
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Defines product's variants specified by variant id
    #[serde(rename = "product_variant_id")]
    pub product_variant_id: String,
    /// Defines image's name
    #[serde(rename = "image_name")]
    pub image_name: String,
    /// Defines image's types that are specified by comma-separated list
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Defines URL of the image that has to be added
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Content(body) encoded in base64 of image file
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Defines alternative text that has to be attached to the picture
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Mime type of image http://en.wikipedia.org/wiki/Internet_media_type.
    #[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    /// Defines image’s position in the list
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Store Id
    #[serde(rename = "store_id", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// Defines option id of the product variant for which the image will be added
    #[serde(rename = "option_id", skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
}

impl ProductVariantImageAdd {
    pub fn new(product_variant_id: String, image_name: String, r#type: Type) -> ProductVariantImageAdd {
        ProductVariantImageAdd {
            product_id: None,
            product_variant_id,
            image_name,
            r#type,
            url: None,
            content: None,
            label: None,
            mime: None,
            position: None,
            store_id: None,
            option_id: None,
        }
    }
}
/// Defines image's types that are specified by comma-separated list
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "additional")]
    Additional,
    #[serde(rename = "thumbnail")]
    Thumbnail,
}

impl Default for Type {
    fn default() -> Type {
        Self::Small
    }
}


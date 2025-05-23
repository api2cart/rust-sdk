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
pub struct ProductUpdate {
    /// Defines product id that has to be updated
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Defines product model that has to be updated
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Defines new product's sku
    #[serde(rename = "sku", skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Defines product's name that has to be updated
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Defines new product's description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Defines short description
    #[serde(rename = "short_description", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    /// Defines new product's price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Defines product's old price
    #[serde(rename = "old_price", skip_serializing_if = "Option::is_none")]
    pub old_price: Option<f64>,
    /// Defines new product's special price
    #[serde(rename = "special_price", skip_serializing_if = "Option::is_none")]
    pub special_price: Option<f64>,
    /// Defines the date of special price creation
    #[serde(rename = "sprice_create", skip_serializing_if = "Option::is_none")]
    pub sprice_create: Option<String>,
    /// Defines the term of special price offer duration
    #[serde(rename = "sprice_expire", skip_serializing_if = "Option::is_none")]
    pub sprice_expire: Option<String>,
    /// Defines new product's cost price
    #[serde(rename = "cost_price", skip_serializing_if = "Option::is_none")]
    pub cost_price: Option<f64>,
    /// Specifies product's fixed cost shipping price
    #[serde(rename = "fixed_cost_shipping_price", skip_serializing_if = "Option::is_none")]
    pub fixed_cost_shipping_price: Option<f64>,
    /// Defines new product's retail price
    #[serde(rename = "retail_price", skip_serializing_if = "Option::is_none")]
    pub retail_price: Option<f64>,
    /// Defines product's tier prices
    #[serde(rename = "tier_prices", skip_serializing_if = "Option::is_none")]
    pub tier_prices: Option<Vec<models::ProductAddTierPricesInner>>,
    /// Defines reserve price value
    #[serde(rename = "reserve_price", skip_serializing_if = "Option::is_none")]
    pub reserve_price: Option<f64>,
    /// Defines buy it now value
    #[serde(rename = "buyitnow_price", skip_serializing_if = "Option::is_none")]
    pub buyitnow_price: Option<f64>,
    /// Specifies whether a tax is charged
    #[serde(rename = "taxable", skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    /// Defines tax classes where entity has to be added
    #[serde(rename = "tax_class_id", skip_serializing_if = "Option::is_none")]
    pub tax_class_id: Option<String>,
    /// Defines product's type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Defines product's status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The human-readable label for the condition (e.g., \"New\").
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// Set visibility status
    #[serde(rename = "visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<String>,
    /// Set stock status
    #[serde(rename = "in_stock", skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,
    /// Defines category's visibility status
    #[serde(rename = "avail", skip_serializing_if = "Option::is_none")]
    pub avail: Option<bool>,
    /// Allows to schedule a time in the future that the item becomes available. The value should be greater than the current date and time.
    #[serde(rename = "avail_from", skip_serializing_if = "Option::is_none")]
    pub avail_from: Option<String>,
    /// A categorization for the product
    #[serde(rename = "product_class", skip_serializing_if = "Option::is_none")]
    pub product_class: Option<String>,
    /// Specifies the set of visible/invisible products for users
    #[serde(rename = "available_for_view", skip_serializing_if = "Option::is_none")]
    pub available_for_view: Option<bool>,
    /// Assign product to the stores that is specified by comma-separated stores' id
    #[serde(rename = "stores_ids", skip_serializing_if = "Option::is_none")]
    pub stores_ids: Option<String>,
    /// Defines store id where the product should be found
    #[serde(rename = "store_id", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// Language id
    #[serde(rename = "lang_id", skip_serializing_if = "Option::is_none")]
    pub lang_id: Option<String>,
    /// Defines new product's quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    /// This parameter allows to reserve/unreserve product quantity.
    #[serde(rename = "reserve_quantity", skip_serializing_if = "Option::is_none")]
    pub reserve_quantity: Option<f64>,
    /// Defines inventory tracking for product
    #[serde(rename = "manage_stock", skip_serializing_if = "Option::is_none")]
    pub manage_stock: Option<bool>,
    /// Set backorder status
    #[serde(rename = "backorder_status", skip_serializing_if = "Option::is_none")]
    pub backorder_status: Option<String>,
    /// Defines the incremental changes in product quantity
    #[serde(rename = "increase_quantity", skip_serializing_if = "Option::is_none")]
    pub increase_quantity: Option<f64>,
    /// Defines the decrement changes in product quantity
    #[serde(rename = "reduce_quantity", skip_serializing_if = "Option::is_none")]
    pub reduce_quantity: Option<f64>,
    /// This parameter is used for selecting a warehouse where you need to set/modify a product quantity.
    #[serde(rename = "warehouse_id", skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    /// Weight
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    /// Weight Unit
    #[serde(rename = "weight_unit", skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
    /// Defines product's height
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// Defines product's length
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    /// Defines product's width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// Weight Unit
    #[serde(rename = "dimensions_unit", skip_serializing_if = "Option::is_none")]
    pub dimensions_unit: Option<String>,
    /// Defines whether the product is virtual
    #[serde(rename = "is_virtual", skip_serializing_if = "Option::is_none")]
    pub is_virtual: Option<bool>,
    /// Specifies product free shipping flag that has to be updated
    #[serde(rename = "is_free_shipping", skip_serializing_if = "Option::is_none")]
    pub is_free_shipping: Option<bool>,
    /// Global Trade Item Number. An GTIN is an identifier for trade items.
    #[serde(rename = "gtin", skip_serializing_if = "Option::is_none")]
    pub gtin: Option<String>,
    /// Universal Product Code. A UPC (UPC-A) is a commonly used identifer for many different products.
    #[serde(rename = "upc", skip_serializing_if = "Option::is_none")]
    pub upc: Option<String>,
    /// Manufacturer Part Number. A MPN is an identifier of a particular part design or material used.
    #[serde(rename = "mpn", skip_serializing_if = "Option::is_none")]
    pub mpn: Option<String>,
    /// European Article Number. An EAN is a unique 8 or 13-digit identifier that many industries (such as book publishers) use to identify products.
    #[serde(rename = "ean", skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,
    /// International Standard Book Number. An ISBN is a unique identifier for books.
    #[serde(rename = "isbn", skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    /// A barcode is a unique code composed of numbers used as a product identifier.
    #[serde(rename = "barcode", skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    /// Defines product's manufacturer
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Defines product's manufacturer by manufacturer_id
    #[serde(rename = "manufacturer_id", skip_serializing_if = "Option::is_none")]
    pub manufacturer_id: Option<String>,
    /// Defines product add that is specified by comma-separated categories id
    #[serde(rename = "categories_ids", skip_serializing_if = "Option::is_none")]
    pub categories_ids: Option<String>,
    /// Defines product related products ids that has to be updated
    #[serde(rename = "related_products_ids", skip_serializing_if = "Option::is_none")]
    pub related_products_ids: Option<String>,
    /// Defines product up-sell products ids that has to be updated
    #[serde(rename = "up_sell_products_ids", skip_serializing_if = "Option::is_none")]
    pub up_sell_products_ids: Option<String>,
    /// Defines product cross-sells products ids that has to be updated
    #[serde(rename = "cross_sell_products_ids", skip_serializing_if = "Option::is_none")]
    pub cross_sell_products_ids: Option<String>,
    /// Defines unique meta title for each entity
    #[serde(rename = "meta_title", skip_serializing_if = "Option::is_none")]
    pub meta_title: Option<String>,
    /// Defines unique meta keywords for each entity
    #[serde(rename = "meta_keywords", skip_serializing_if = "Option::is_none")]
    pub meta_keywords: Option<String>,
    /// Defines unique meta description of a entity
    #[serde(rename = "meta_description", skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,
    /// Defines unique URL for SEO
    #[serde(rename = "seo_url", skip_serializing_if = "Option::is_none")]
    pub seo_url: Option<String>,
    /// Defines unique search keywords
    #[serde(rename = "search_keywords", skip_serializing_if = "Option::is_none")]
    pub search_keywords: Option<String>,
    /// Product tags
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// The delivery promise that applies to offer
    #[serde(rename = "delivery_code", skip_serializing_if = "Option::is_none")]
    pub delivery_code: Option<String>,
    #[serde(rename = "package_details", skip_serializing_if = "Option::is_none")]
    pub package_details: Option<Box<models::ProductAddPackageDetails>>,
    /// The country where the inventory item was made
    #[serde(rename = "country_of_origin", skip_serializing_if = "Option::is_none")]
    pub country_of_origin: Option<String>,
    /// Harmonized System Code. An HSC is a 6-digit identifier that allows participating countries to classify traded goods on a common basis for customs purposes
    #[serde(rename = "harmonized_system_code", skip_serializing_if = "Option::is_none")]
    pub harmonized_system_code: Option<String>,
    /// The numeric ID of the shipping template associated with the products in Etsy. You can find possible values in the \"cart.info\" API method response, in the field shipping_zones[]->id.
    #[serde(rename = "shipping_template_id", skip_serializing_if = "Option::is_none")]
    pub shipping_template_id: Option<i32>,
    /// An enumerated string for the era in which the maker made the product.
    #[serde(rename = "when_made", skip_serializing_if = "Option::is_none")]
    pub when_made: Option<String>,
    /// If true, it indicates the product as a supply, otherwise it indicates that it is a finished product.
    #[serde(rename = "is_supply", skip_serializing_if = "Option::is_none")]
    pub is_supply: Option<bool>,
    /// Defines whether the product is downloadable
    #[serde(rename = "downloadable", skip_serializing_if = "Option::is_none")]
    pub downloadable: Option<bool>,
    /// A list of material strings for materials used in the product.
    #[serde(rename = "materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<String>>,
    /// When true, automatically renews a listing upon its expiration.
    #[serde(rename = "auto_renew", skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// Set whether the product on sale
    #[serde(rename = "on_sale", skip_serializing_if = "Option::is_none")]
    pub on_sale: Option<bool>,
    /// Defines product production partner ids that has to be updated
    #[serde(rename = "production_partner_ids", skip_serializing_if = "Option::is_none")]
    pub production_partner_ids: Option<String>,
    #[serde(rename = "manufacturer_info", skip_serializing_if = "Option::is_none")]
    pub manufacturer_info: Option<Box<models::ProductAddManufacturerInfo>>,
    /// Report request id
    #[serde(rename = "report_request_id", skip_serializing_if = "Option::is_none")]
    pub report_request_id: Option<String>,
    /// Disable report cache for current request
    #[serde(rename = "disable_report_cache", skip_serializing_if = "Option::is_none")]
    pub disable_report_cache: Option<bool>,
    /// Is reindex required
    #[serde(rename = "reindex", skip_serializing_if = "Option::is_none")]
    pub reindex: Option<bool>,
    /// Is cache clear required
    #[serde(rename = "clear_cache", skip_serializing_if = "Option::is_none")]
    pub clear_cache: Option<bool>,
    /// Disable or enable check process status. Please note that the response will be slower due to additional requests to the store.
    #[serde(rename = "check_process_status", skip_serializing_if = "Option::is_none")]
    pub check_process_status: Option<bool>,
    /// An array of Item Specific Name/Value pairs used by the seller to provide descriptive details of an item in a structured manner.         The list of possible specifications can be obtained using the category.info method (additional_fields->product_specifics).         <b>The structure of the parameter is different for specific platforms.</b>
    #[serde(rename = "specifics", skip_serializing_if = "Option::is_none")]
    pub specifics: Option<Vec<models::ProductAddSpecificsInner>>,
    /// Add Shop Section Id
    #[serde(rename = "shop_section_id", skip_serializing_if = "Option::is_none")]
    pub shop_section_id: Option<i32>,
    #[serde(rename = "personalization_details", skip_serializing_if = "Option::is_none")]
    pub personalization_details: Option<Box<models::ProductAddPersonalizationDetails>>,
}

impl ProductUpdate {
    pub fn new() -> ProductUpdate {
        ProductUpdate {
            id: None,
            model: None,
            sku: None,
            name: None,
            description: None,
            short_description: None,
            price: None,
            old_price: None,
            special_price: None,
            sprice_create: None,
            sprice_expire: None,
            cost_price: None,
            fixed_cost_shipping_price: None,
            retail_price: None,
            tier_prices: None,
            reserve_price: None,
            buyitnow_price: None,
            taxable: None,
            tax_class_id: None,
            r#type: None,
            status: None,
            condition: None,
            visible: None,
            in_stock: None,
            avail: None,
            avail_from: None,
            product_class: None,
            available_for_view: None,
            stores_ids: None,
            store_id: None,
            lang_id: None,
            quantity: None,
            reserve_quantity: None,
            manage_stock: None,
            backorder_status: None,
            increase_quantity: None,
            reduce_quantity: None,
            warehouse_id: None,
            weight: None,
            weight_unit: None,
            height: None,
            length: None,
            width: None,
            dimensions_unit: None,
            is_virtual: None,
            is_free_shipping: None,
            gtin: None,
            upc: None,
            mpn: None,
            ean: None,
            isbn: None,
            barcode: None,
            manufacturer: None,
            manufacturer_id: None,
            categories_ids: None,
            related_products_ids: None,
            up_sell_products_ids: None,
            cross_sell_products_ids: None,
            meta_title: None,
            meta_keywords: None,
            meta_description: None,
            seo_url: None,
            search_keywords: None,
            tags: None,
            delivery_code: None,
            package_details: None,
            country_of_origin: None,
            harmonized_system_code: None,
            shipping_template_id: None,
            when_made: None,
            is_supply: None,
            downloadable: None,
            materials: None,
            auto_renew: None,
            on_sale: None,
            production_partner_ids: None,
            manufacturer_info: None,
            report_request_id: None,
            disable_report_cache: None,
            reindex: None,
            clear_cache: None,
            check_process_status: None,
            specifics: None,
            shop_section_id: None,
            personalization_details: None,
        }
    }
}


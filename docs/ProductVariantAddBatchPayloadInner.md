# ProductVariantAddBatchPayloadInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | **String** |  | 
**combination** | [**Vec<models::ProductVariantAddBatchPayloadInnerCombinationInner>**](ProductVariantAddBatch_payload_inner_combination_inner.md) | A unique combination that contains an array of options and their values, which form a variation. | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**short_description** | Option<**String**> |  | [optional]
**sku** | **String** |  | 
**model** | Option<**String**> |  | [optional]
**price** | Option<**f64**> |  | [optional]
**old_price** | Option<**f64**> |  | [optional]
**cost_price** | Option<**f64**> |  | [optional]
**special_price** | Option<**f64**> |  | [optional]
**sprice_create** | Option<**String**> |  | [optional]
**sprice_expire** | Option<**String**> |  | [optional]
**advanced_prices** | Option<[**Vec<models::ProductUpdateBatchPayloadInnerAdvancedPricesInner>**](ProductUpdateBatch_payload_inner_advanced_prices_inner.md)> |  | [optional]
**meta_title** | Option<**f64**> |  | [optional]
**meta_description** | Option<**f64**> |  | [optional]
**meta_keywords** | Option<**Vec<String>**> |  | [optional]
**categories_ids** | Option<**Vec<String>**> |  | [optional]
**stores_ids** | Option<**Vec<String>**> |  | [optional]
**weight** | Option<**f64**> |  | [optional]
**width** | Option<**f64**> |  | [optional]
**height** | Option<**f64**> |  | [optional]
**length** | Option<**f64**> |  | [optional]
**weight_unit** | Option<**String**> |  | [optional]
**warehouse_id** | Option<**String**> |  | [optional]
**quantity** | Option<**f64**> |  | [optional]
**manage_stock** | Option<**bool**> |  | [optional]
**in_stock** | Option<**bool**> |  | [optional]
**store_id** | Option<**String**> |  | [optional]
**lang_id** | Option<**String**> |  | [optional]
**tax_class_id** | Option<**String**> |  | [optional]
**backorder_status** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**visible** | Option<**String**> |  | [optional]
**is_virtual** | Option<**bool**> |  | [optional]
**downloadable** | Option<**bool**> |  | [optional]
**is_default** | Option<**bool**> |  | [optional]
**upc** | Option<**String**> |  | [optional]
**isbn** | Option<**String**> |  | [optional]
**mpn** | Option<**String**> |  | [optional]
**ean** | Option<**String**> |  | [optional]
**barcode** | Option<**String**> |  | [optional]
**available_for_sale** | Option<**bool**> |  | [optional]
**is_free_shipping** | Option<**bool**> |  | [optional]
**taxable** | Option<**bool**> |  | [optional]
**seo_url** | Option<**String**> |  | [optional]
**manufacturer_id** | Option<**String**> |  | [optional]
**harmonized_system_code** | Option<**String**> |  | [optional]
**marketplace_item_properties** | Option<[**serde_json::Value**](.md)> |  | [optional]
**images** | Option<[**Vec<models::ProductAddBatchPayloadInnerImagesInner>**](ProductAddBatch_payload_inner_images_inner.md)> |  | [optional]
**product_images_ids** | Option<**Vec<String>**> |  | [optional]
**related_products_ids** | Option<**Vec<String>**> |  | [optional]
**up_sell_products_ids** | Option<**Vec<String>**> |  | [optional]
**cross_sell_products_ids** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



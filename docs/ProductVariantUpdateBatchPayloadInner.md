# ProductVariantUpdateBatchPayloadInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**product_id** | **String** |  | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**short_description** | Option<**String**> |  | [optional]
**sku** | Option<**String**> |  | [optional]
**upc** | Option<**String**> |  | [optional]
**mpn** | Option<**String**> |  | [optional]
**gtin** | Option<**String**> |  | [optional]
**isbn** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**price** | Option<**f64**> |  | [optional]
**special_price** | Option<**f64**> |  | [optional]
**cost_price** | Option<**f64**> |  | [optional]
**retail_price** | Option<**f64**> |  | [optional]
**advanced_prices** | Option<[**Vec<models::ProductUpdateBatchPayloadInnerAdvancedPricesInner>**](ProductUpdateBatch_payload_inner_advanced_prices_inner.md)> | If an empty array is passed, all entries will be deleted when the 'nested_items_update_behaviour' parameter is set to 'replace'. | [optional]
**quantity** | Option<**f64**> |  | [optional]
**reserve_quantity** | Option<**f64**> |  | [optional]
**increase_quantity** | Option<**f64**> |  | [optional]
**reduce_quantity** | Option<**f64**> |  | [optional]
**warehouse_id** | Option<**String**> |  | [optional]
**manufacturer_id** | Option<**String**> |  | [optional]
**weight** | Option<**f64**> |  | [optional]
**height** | Option<**f64**> |  | [optional]
**length** | Option<**f64**> |  | [optional]
**width** | Option<**f64**> |  | [optional]
**store_id** | Option<**String**> |  | [optional]
**lang_id** | Option<**String**> |  | [optional]
**tax_class_id** | Option<**String**> |  | [optional]
**backorder_status** | Option<**String**> |  | [optional]
**visible** | Option<**String**> |  | [optional]
**is_default** | Option<**bool**> |  | [optional]
**in_stock** | Option<**bool**> |  | [optional]
**is_virtual** | Option<**bool**> |  | [optional]
**downloadable** | Option<**bool**> |  | [optional]
**manage_stock** | Option<**bool**> |  | [optional]
**is_free_shipping** | Option<**bool**> |  | [optional]
**seo_url** | Option<**String**> |  | [optional]
**meta_title** | Option<**String**> |  | [optional]
**meta_description** | Option<**String**> |  | [optional]
**meta_keywords** | Option<**Vec<String>**> |  | [optional]
**categories_ids** | Option<**Vec<String>**> | If an empty array is passed, all entries will be deleted when the 'nested_items_update_behaviour' parameter is set to 'replace'. | [optional]
**stores_ids** | Option<**Vec<String>**> |  | [optional]
**images** | Option<[**Vec<models::ProductAddBatchPayloadInnerImagesInner>**](ProductAddBatch_payload_inner_images_inner.md)> | The passed items will completely replace the original items | [optional]
**product_images_ids** | Option<**Vec<String>**> |  | [optional]
**related_products_ids** | Option<**Vec<String>**> | If an empty array is passed, all entries will be deleted when the 'nested_items_update_behaviour' parameter is set to 'replace'. | [optional]
**up_sell_products_ids** | Option<**Vec<String>**> | If an empty array is passed, all entries will be deleted when the 'nested_items_update_behaviour' parameter is set to 'replace'. | [optional]
**cross_sell_products_ids** | Option<**Vec<String>**> | If an empty array is passed, all entries will be deleted when the 'nested_items_update_behaviour' parameter is set to 'replace'. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ProductVariantPriceUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Defines the variant where the price has to be updated | [optional]
**product_id** | Option<**String**> | Product id | [optional]
**group_prices** | [**Vec<models::ProductPriceUpdateGroupPricesInner>**](ProductPriceUpdate_group_prices_inner.md) | Defines variants's group prices | 
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



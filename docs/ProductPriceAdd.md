# ProductPriceAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | Option<**String**> | Defines the product to which the price has to be added | [optional]
**group_prices** | Option<[**Vec<models::ProductAddGroupPricesInner>**](ProductAdd_group_prices_inner.md)> | Defines product's group prices | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



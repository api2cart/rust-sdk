# ProductTaxAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | Option<**String**> | Defines products specified by product id | [optional]
**name** | **String** | Defines tax class name where tax has to be added | 
**tax_rates** | [**Vec<models::ProductTaxAddTaxRatesInner>**](ProductTaxAdd_tax_rates_inner.md) | Defines tax rates of specified tax classes | 
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



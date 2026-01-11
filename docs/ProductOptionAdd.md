# ProductOptionAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Defines option's name | 
**r#type** | **String** | Defines option's type that has to be added | 
**product_id** | Option<**String**> | Defines product id where the option should be added | [optional]
**default_option_value** | Option<**String**> | Defines default option value that has to be added | [optional]
**option_values** | Option<**String**> | Defines option values that has to be added | [optional]
**description** | Option<**String**> | Defines option's description | [optional]
**avail** | Option<**bool**> | Defines whether the option is available | [optional][default to true]
**sort_order** | Option<**i32**> | Sort number in the list | [optional][default to 0]
**required** | Option<**bool**> | Defines if the option is required | [optional][default to false]
**values** | Option<[**Vec<models::ProductOptionAddValuesInner>**](ProductOptionAdd_values_inner.md)> | An array of option values.</b> | [optional]
**clear_cache** | Option<**bool**> | Is cache clear required | [optional][default to true]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



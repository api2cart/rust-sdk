# OrderPreestimateShippingList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**warehouse_id** | Option<**String**> | This parameter is used for selecting a warehouse where you need to set/modify a product quantity. | [optional]
**customer_id** | Option<**String**> | Retrieves orders specified by customer id | [optional]
**customer_email** | Option<**String**> | Retrieves orders specified by customer email | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**shipp_address_1** | Option<**String**> | Specifies first shipping address | [optional]
**shipp_city** | Option<**String**> | Specifies shipping city | [optional]
**shipp_postcode** | Option<**String**> | Specifies shipping postcode | [optional]
**shipp_state** | Option<**String**> | Specifies shipping state code | [optional]
**shipp_country** | **String** | Specifies shipping country code | 
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve | [optional][default to force_all]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all | [optional]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]
**order_item** | [**Vec<models::OrderPreestimateShippingListOrderItemInner>**](OrderPreestimateShippingList_order_item_inner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



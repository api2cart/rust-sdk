# OrderReturnUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**return_id** | **String** | Return ID | 
**order_id** | Option<**String**> | Defines the order id | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**item_restock** | Option<**bool**> | Boolean, whether or not to add the line items back to the store inventory. | [optional][default to false]
**return_status_id** | Option<**String**> | Defines return request status | [optional]
**staff_note** | Option<**String**> | Specifies staff note | [optional]
**comment** | Option<**String**> | Specifies return comment | [optional]
**send_notifications** | Option<**bool**> | Send notifications to customer after order was created | [optional][default to false]
**reject_reason** | Option<**String**> | Defines return reject reason | [optional]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]
**order_products** | [**Vec<models::OrderReturnUpdateOrderProductsInner>**](OrderReturnUpdate_order_products_inner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



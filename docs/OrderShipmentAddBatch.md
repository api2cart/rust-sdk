# OrderShipmentAddBatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payload** | [**Vec<models::OrderShipmentAddBatchPayloadInner>**](OrderShipmentAddBatch_payload_inner.md) | Contains an array of order shipment objects. The list of properties may vary depending on the specific platform. | 
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# OrderShipmentTrackingAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | Option<**String**> | Defines the order id | [optional]
**shipment_id** | **String** | Shipment id indicates the number of delivery | 
**carrier_id** | Option<**String**> | Defines tracking carrier id | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**tracking_provider** | Option<**String**> | Defines name of the company which provides shipment tracking | [optional]
**tracking_number** | **String** | Defines tracking number | 
**tracking_link** | Option<**String**> | Defines custom tracking link | [optional]
**send_notifications** | Option<**bool**> | Send notifications to customer after tracking was created | [optional][default to false]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



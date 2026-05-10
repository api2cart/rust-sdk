# OrderShipmentEventAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | Defines the shipment to which the tracking event will be added | 
**order_id** | Option<**String**> | Defines the order to which the shipment belongs | [optional]
**status** | **String** | Defines the tracking event status (e.g. in_transit, delivered, out_for_delivery) | 
**store_id** | Option<**String**> | Store Id | [optional]
**address_1** | Option<**String**> | Specifies the street address of the event location | [optional]
**city** | Option<**String**> | Specifies city | [optional]
**country** | Option<**String**> | Specifies ISO code or name of country | [optional]
**state** | Option<**String**> | Specifies ISO code or name of state | [optional]
**postcode** | Option<**String**> | Specifies postcode | [optional]
**message** | Option<**String**> | Defines a message associated with the tracking event. | [optional]
**latitude** | Option<**f64**> | Latitude coordinate of the event location. | [optional]
**longitude** | Option<**f64**> | Longitude coordinate of the event location. | [optional]
**created_at** | Option<**String**> | Defines the date of entity creation | [optional]
**estimated_delivery_at** | Option<**String**> | Estimated delivery date and time in ISO 8601 format. | [optional]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



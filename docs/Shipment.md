# Shipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**order_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**warehouse_id** | Option<**String**> |  | [optional]
**shipment_provider** | Option<**String**> |  | [optional]
**tracking_numbers** | Option<[**Vec<models::ShipmentTrackingNumber>**](Shipment_TrackingNumber.md)> |  | [optional]
**created_at** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**modified_time** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**items** | Option<[**Vec<models::ShipmentItem>**](Shipment_Item.md)> |  | [optional]
**is_shipped** | Option<**bool**> |  | [optional]
**delivered_at** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**additional_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



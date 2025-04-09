# OrderStatusRefund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipping** | Option<**f64**> |  | [optional]
**fee** | Option<**f64**> |  | [optional]
**tax** | Option<**f64**> |  | [optional]
**total_refunded** | Option<**f64**> |  | [optional]
**time** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**refunded_items** | Option<[**Vec<models::OrderStatusRefundItem>**](Order_Status_Refund_Item.md)> |  | [optional]
**additional_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



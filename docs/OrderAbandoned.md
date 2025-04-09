# OrderAbandoned

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**customer** | Option<[**models::BaseCustomer**](BaseCustomer.md)> |  | [optional]
**basket_id** | Option<**String**> |  | [optional]
**basket_url** | Option<**String**> |  | [optional]
**created_at** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**modified_at** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**currency** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**totals** | Option<[**models::OrderTotals**](Order_Totals.md)> |  | [optional]
**order_products** | Option<[**Vec<models::OrderItem>**](Order_Item.md)> |  | [optional]
**additional_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



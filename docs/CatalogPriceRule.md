# CatalogPriceRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**gid** | Option<**String**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**short_description** | Option<**String**> |  | [optional]
**avail** | Option<**bool**> |  | [optional]
**actions** | Option<[**Vec<models::CatalogPriceRuleAction>**](CatalogPriceRule_Action.md)> |  | [optional]
**created_time** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**date_start** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**date_end** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**usage_count** | Option<**f64**> |  | [optional]
**conditions** | Option<[**Vec<models::CouponCondition>**](Coupon_Condition.md)> |  | [optional]
**uses_per_order_limit** | Option<**i32**> |  | [optional]
**additional_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



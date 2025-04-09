# Coupon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**code** | Option<**String**> |  | [optional]
**codes** | Option<[**Vec<models::CouponCode>**](Coupon_Code.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**actions** | Option<[**Vec<models::CouponAction>**](Coupon_Action.md)> |  | [optional]
**date_start** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**date_end** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**avail** | Option<**bool**> |  | [optional]
**priority** | Option<**i32**> |  | [optional]
**used_times** | Option<**i32**> |  | [optional]
**usage_limit** | Option<**i32**> |  | [optional]
**usage_limit_per_customer** | Option<**i32**> |  | [optional]
**logic_operator** | Option<**String**> |  | [optional]
**conditions** | Option<[**Vec<models::CouponCondition>**](Coupon_Condition.md)> |  | [optional]
**usage_history** | Option<[**Vec<models::CouponHistory>**](Coupon_History.md)> |  | [optional]
**additional_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



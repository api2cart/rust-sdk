# CartCouponAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | Coupon code | 
**action_type** | **String** | Coupon discount type | 
**action_apply_to** | **String** | Defines where discount should be applied | 
**action_scope** | **String** | Specify how discount should be applied. If scope=matching_items, then discount will be applied to each of the items that match action conditions. Scope order means that discount will be applied once. | 
**action_amount** | **f64** | Defines the discount amount value. | 
**codes** | Option<**Vec<String>**> | Entity codes | [optional]
**name** | Option<**String**> | Coupon name | [optional]
**date_start** | Option<**String**> | Date start | [optional][default to now]
**date_end** | Option<**String**> | Defines when discount code will be expired. | [optional]
**usage_limit** | Option<**i32**> | Usage limit for coupon. | [optional]
**usage_limit_per_customer** | Option<**i32**> | Usage limit per customer. | [optional]
**action_condition_entity** | Option<**String**> | Defines entity for action condition. | [optional]
**action_condition_key** | Option<**String**> | Defines entity attribute code for action condition. | [optional]
**action_condition_operator** | Option<**String**> | Defines condition operator. | [optional]
**action_condition_value** | Option<**String**> | Defines condition attribute value/s. Can be comma separated string. | [optional]
**include_tax** | Option<**bool**> | Indicates whether to apply a discount for taxes. | [optional][default to false]
**store_id** | Option<**String**> | Store Id | [optional]
**free_cash_on_delivery** | Option<**bool**> | Defines whether the coupon provides free cash on delivery | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



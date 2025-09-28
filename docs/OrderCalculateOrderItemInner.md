# OrderCalculateOrderItemInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_item_id** | **String** | Defines orders specified by order item id | 
**order_item_quantity** | **i32** | Defines orders specified by order item quantity | 
**order_item_variant_id** | Option<**String**> | Ordered product variant. Where x is order item ID | [optional]
**order_item_parent** | Option<**i32**> | Index of the parent grouped/bundle product | [optional]
**order_item_parent_option_name** | Option<**String**> | Option name of the parent grouped/bundle product | [optional]
**order_item_option** | Option<[**Vec<models::OrderCalculateOrderItemInnerOrderItemOptionInner>**](OrderCalculate_order_item_inner_order_item_option_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



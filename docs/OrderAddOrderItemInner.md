# OrderAddOrderItemInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_item_id** | **String** | Defines orders specified by order item id | 
**order_item_name** | **String** | Defines orders specified by order item name | 
**order_item_model** | Option<**String**> | Defines orders specified by order item model | [optional]
**order_item_price** | **f64** | Defines orders specified by order item price | 
**order_item_quantity** | **i32** | Defines orders specified by order item quantity | 
**order_item_weight** | Option<**f64**> | Defines orders specified by order item weight | [optional]
**order_item_variant_id** | Option<**String**> | Ordered product variant. Where x is order item ID | [optional]
**order_item_tax** | Option<**f64**> | Percentage of tax for product order | [optional][default to 0]
**order_item_parent** | Option<**i32**> | Index of the parent grouped/bundle product | [optional]
**order_item_parent_option_name** | Option<**String**> | Option name of the parent grouped/bundle product | [optional]
**order_item_allow_refund_items_separately** | Option<**bool**> | Indicates whether subitems of the grouped/bundle product can be refunded separately | [optional]
**order_item_allow_ship_items_separately** | Option<**bool**> | Indicates whether subitems of the grouped/bundle product can be shipped separately | [optional]
**order_item_price_includes_tax** | Option<**bool**> | Defines if item price includes tax | [optional][default to false]
**order_item_option** | Option<[**Vec<models::OrderAddOrderItemInnerOrderItemOptionInner>**](OrderAdd_order_item_inner_order_item_option_inner.md)> |  | [optional]
**order_item_property** | Option<[**Vec<models::OrderAddOrderItemInnerOrderItemPropertyInner>**](OrderAdd_order_item_inner_order_item_property_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



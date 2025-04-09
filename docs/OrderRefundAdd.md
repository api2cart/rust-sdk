# OrderRefundAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | Option<**String**> | Defines the order for which the refund will be created. | [optional]
**items** | Option<[**Vec<models::OrderRefundAddItemsInner>**](OrderRefundAdd_items_inner.md)> | Defines items in the order that will be refunded | [optional]
**total_price** | Option<**f64**> | Defines order refund amount. | [optional]
**shipping_price** | Option<**f64**> | Defines refund shipping amount. | [optional]
**fee_price** | Option<**f64**> | Specifies refund's fee price | [optional]
**message** | Option<**String**> | Refund reason, or some else message which assigned to refund. | [optional]
**item_restock** | Option<**bool**> | Boolean, whether or not to add the line items back to the store inventory. | [optional][default to false]
**send_notifications** | Option<**bool**> | Send notifications to customer after refund was created | [optional][default to false]
**date** | Option<**String**> | Specifies an order creation date in format Y-m-d H:i:s | [optional]
**is_online** | Option<**bool**> | Indicates whether refund type is online | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**order_id** | Option<**String**> |  | [optional]
**basket_id** | Option<**String**> |  | [optional]
**channel_id** | Option<**String**> |  | [optional]
**customer** | Option<[**models::BaseCustomer**](BaseCustomer.md)> |  | [optional]
**create_at** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**currency** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**shipping_address** | Option<[**models::CustomerAddress**](Customer_Address.md)> |  | [optional]
**billing_address** | Option<[**models::CustomerAddress**](Customer_Address.md)> |  | [optional]
**payment_method** | Option<[**models::OrderPaymentMethod**](Order_PaymentMethod.md)> |  | [optional]
**shipping_method** | Option<[**models::OrderShippingMethod**](Order_ShippingMethod.md)> |  | [optional]
**shipping_methods** | Option<[**Vec<models::OrderShippingMethod>**](Order_ShippingMethod.md)> |  | [optional]
**status** | Option<[**models::OrderStatus**](Order_Status.md)> |  | [optional]
**totals** | Option<[**models::OrderTotals**](Order_Totals.md)> |  | [optional]
**total** | Option<[**models::OrderTotal**](Order_Total.md)> |  | [optional]
**discounts** | Option<[**Vec<models::OrderTotalsNewDiscount>**](Order_Totals_NewDiscount.md)> |  | [optional]
**order_products** | Option<[**Vec<models::OrderItem>**](Order_Item.md)> |  | [optional]
**bundles** | Option<[**Vec<models::OrderItem>**](Order_Item.md)> |  | [optional]
**modified_at** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**finished_time** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**store_id** | Option<**String**> |  | [optional]
**warehouses_ids** | Option<**Vec<String>**> |  | [optional]
**refunds** | Option<[**Vec<models::OrderRefund>**](Order_Refund.md)> |  | [optional]
**gift_message** | Option<**String**> |  | [optional]
**order_details_url** | Option<**String**> |  | [optional]
**additional_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



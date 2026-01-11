# OrderAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Defines order's id | [optional]
**order_id** | Option<**String**> | Defines the order id if it is supported by the cart | [optional]
**store_id** | Option<**String**> | Defines store id where the order should be assigned | [optional]
**channel_id** | Option<**String**> | Channel ID | [optional]
**order_status** | **String** | Defines order status. | 
**fulfillment_status** | Option<**String**> | Create order with fulfillment status | [optional]
**financial_status** | Option<**String**> | Create order with financial status | [optional]
**customer_email** | **String** | Defines the customer specified by email for whom order has to be created | 
**customer_first_name** | Option<**String**> | Specifies customer's first name | [optional]
**customer_last_name** | Option<**String**> | Specifies customer’s last name | [optional]
**customer_phone** | Option<**String**> | Specifies customer’s phone | [optional]
**customer_country** | Option<**String**> | Specifies customer's address ISO code or name of country | [optional]
**customer_birthday** | Option<**String**> | Specifies customer’s birthday | [optional]
**customer_fax** | Option<**String**> | Specifies customer’s fax | [optional]
**is_guest** | Option<**bool**> | Indicates whether the customer is a guest customer | [optional][default to false]
**order_payment_method** | Option<**String**> | Defines order payment method.<br/>Setting order_payment_method on Shopify will also change financial_status field value to 'paid' | [optional]
**transaction_id** | Option<**String**> | Payment transaction id | [optional]
**currency** | Option<**String**> | Currency code of order | [optional]
**date** | Option<**String**> | Specifies an order creation date in format Y-m-d H:i:s | [optional]
**date_modified** | Option<**String**> | Specifies order's  modification date | [optional]
**date_finished** | Option<**String**> | Specifies order's  finished date | [optional]
**bill_first_name** | **String** | Specifies billing first name | 
**bill_last_name** | **String** | Specifies billing last name | 
**bill_address_1** | **String** | Specifies first billing address | 
**bill_address_2** | Option<**String**> | Specifies second billing address | [optional]
**bill_city** | **String** | Specifies billing city | 
**bill_postcode** | **String** | Specifies billing postcode | 
**bill_state** | **String** | Specifies billing state code | 
**bill_country** | **String** | Specifies billing country code | 
**bill_company** | Option<**String**> | Specifies billing company | [optional]
**bill_phone** | Option<**String**> | Specifies billing phone | [optional]
**bill_fax** | Option<**String**> | Specifies billing fax | [optional]
**shipp_first_name** | Option<**String**> | Specifies shipping first name | [optional]
**shipp_last_name** | Option<**String**> | Specifies shipping last name | [optional]
**shipp_address_1** | Option<**String**> | Specifies first shipping address | [optional]
**shipp_address_2** | Option<**String**> | Specifies second address line of a shipping street address | [optional]
**shipp_city** | Option<**String**> | Specifies shipping city | [optional]
**shipp_postcode** | Option<**String**> | Specifies shipping postcode | [optional]
**shipp_state** | Option<**String**> | Specifies shipping state code | [optional]
**shipp_country** | Option<**String**> | Specifies shipping country code | [optional]
**shipp_company** | Option<**String**> | Specifies shipping company | [optional]
**shipp_phone** | Option<**String**> | Specifies shipping phone | [optional]
**shipp_fax** | Option<**String**> | Specifies shipping fax | [optional]
**subtotal_price** | Option<**f64**> | Total price of all ordered products multiplied by their number, excluding tax, shipping price and discounts | [optional]
**tax_price** | Option<**f64**> | The value of tax cost for order | [optional][default to 0]
**total_price** | Option<**f64**> | Defines order's total price | [optional]
**total_paid** | Option<**f64**> | Defines total paid amount for the order | [optional]
**total_weight** | Option<**i32**> | Defines the sum of all line item weights in grams for the order | [optional]
**prices_inc_tax** | Option<**bool**> | Indicates whether prices and subtotal includes tax. | [optional][default to false]
**shipping_price** | Option<**f64**> | Specifies order's shipping price | [optional][default to 0]
**shipping_tax** | Option<**f64**> | Specifies order's shipping price tax | [optional]
**discount** | Option<**f64**> | Specifies order's discount | [optional]
**coupon_discount** | Option<**f64**> | Specifies order's coupon discount | [optional]
**gift_certificate_discount** | Option<**f64**> | Discounts for order with gift certificates | [optional]
**order_shipping_method** | Option<**String**> | Defines order shipping method | [optional]
**carrier_id** | Option<**String**> | Defines tracking carrier id | [optional]
**warehouse_id** | Option<**String**> | This parameter is used for selecting a warehouse where you need to set/modify a product quantity. | [optional]
**coupons** | Option<**Vec<String>**> | Coupons that will be applied to order | [optional]
**tags** | Option<**String**> | Order tags | [optional]
**comment** | Option<**String**> | Specifies order comment | [optional]
**admin_comment** | Option<**String**> | Specifies admin's order comment | [optional]
**admin_private_comment** | Option<**String**> | Specifies private admin's order comment | [optional]
**send_notifications** | Option<**bool**> | Send notifications to customer after order was created | [optional][default to false]
**send_admin_notifications** | Option<**bool**> | Notify admin when new order was created. | [optional][default to false]
**external_source** | Option<**String**> | Identifying the system used to generate the order | [optional]
**inventory_behaviour** | Option<**String**> | The behaviour to use when updating inventory.<hr><div style=\"font-style:normal\">Values description:<div style=\"margin-left: 2%; padding-top: 2%\"><div style=\"font-size:85%\"><b>bypass</b> = Do not claim inventory </br></br><b>decrement_ignoring_policy</b> = Ignore the product's </br> inventory policy and claim amounts</br></br><b>decrement_obeying_policy</b> =  Obey the product's </br> inventory policy.</br></br></div></div></div> | [optional][default to bypass]
**create_invoice** | Option<**bool**> | Defines whether the invoice is created automatically along with the order | [optional][default to false]
**note_attributes** | Option<[**Vec<models::OrderAddNoteAttributesInner>**](OrderAdd_note_attributes_inner.md)> | Defines note attributes | [optional]
**clear_cache** | Option<**bool**> | Is cache clear required | [optional][default to true]
**origin** | Option<**String**> | The source of the order | [optional]
**fee_price** | Option<**f64**> | Specifies refund's fee price | [optional]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]
**order_item** | [**Vec<models::OrderAddOrderItemInner>**](OrderAdd_order_item_inner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



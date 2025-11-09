# OrderCalculate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_email** | **String** | Defines the customer specified by email for whom the order needs to be calculated | 
**currency_id** | Option<**String**> | Currency Id | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**coupons** | Option<**Vec<String>**> | Coupons that will be applied to order. If the order isn't eligible for any given discount code or there is no discount with such a code it will be skipped during calculation | [optional]
**rounding_precision** | Option<**i32**> | <p>Specifies the rounding precision for fractional numeric values (such as prices, taxes, and weights).</p> <p>Supported values range from <b>1</b> to <b>6</b>.</p> <p>The default rounding precision may vary depending on the platform. You can retrieve the default value using the <strong>cart.info</strong> method in the <code>default_rounding_precision</code> field. </p><p>Values are rounded to the nearest number at the specified precision. Fractions of .5 or higher are rounded up, while fractions lower than .5 are rounded down.</p> | [optional]
**shipp_first_name** | **String** | Specifies shipping first name | 
**shipp_last_name** | **String** | Specifies shipping last name | 
**shipp_address_1** | **String** | Specifies first shipping address | 
**shipp_address_2** | Option<**String**> | Specifies second address line of a shipping street address | [optional]
**shipp_city** | **String** | Specifies shipping city | 
**shipp_postcode** | **String** | Specifies shipping postcode | 
**shipp_state** | Option<**String**> | Specifies shipping state code | [optional]
**shipp_country** | **String** | Specifies shipping country code | 
**shipp_company** | Option<**String**> | Specifies shipping company | [optional]
**shipp_phone** | Option<**String**> | Specifies shipping phone | [optional]
**bill_first_name** | Option<**String**> | Specifies billing first name | [optional]
**bill_last_name** | Option<**String**> | Specifies billing last name | [optional]
**bill_address_1** | Option<**String**> | Specifies first billing address | [optional]
**bill_address_2** | Option<**String**> | Specifies second billing address | [optional]
**bill_city** | Option<**String**> | Specifies billing city | [optional]
**bill_postcode** | Option<**String**> | Specifies billing postcode | [optional]
**bill_state** | Option<**String**> | Specifies billing state code | [optional]
**bill_country** | Option<**String**> | Specifies billing country code | [optional]
**bill_company** | Option<**String**> | Specifies billing company | [optional]
**bill_phone** | Option<**String**> | Specifies billing phone | [optional]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve | [optional]
**order_item** | [**Vec<models::OrderCalculateOrderItemInner>**](OrderCalculate_order_item_inner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



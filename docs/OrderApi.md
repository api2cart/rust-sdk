# \OrderApi

All URIs are relative to *https://api.api2cart.local.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**order_abandoned_list**](OrderApi.md#order_abandoned_list) | **GET** /order.abandoned.list.json | order.abandoned.list
[**order_add**](OrderApi.md#order_add) | **POST** /order.add.json | order.add
[**order_calculate**](OrderApi.md#order_calculate) | **POST** /order.calculate.json | order.calculate
[**order_count**](OrderApi.md#order_count) | **GET** /order.count.json | order.count
[**order_financial_status_list**](OrderApi.md#order_financial_status_list) | **GET** /order.financial_status.list.json | order.financial_status.list
[**order_fulfillment_status_list**](OrderApi.md#order_fulfillment_status_list) | **GET** /order.fulfillment_status.list.json | order.fulfillment_status.list
[**order_info**](OrderApi.md#order_info) | **GET** /order.info.json | order.info
[**order_list**](OrderApi.md#order_list) | **GET** /order.list.json | order.list
[**order_preestimate_shipping_list**](OrderApi.md#order_preestimate_shipping_list) | **POST** /order.preestimate_shipping.list.json | order.preestimate_shipping.list
[**order_refund_add**](OrderApi.md#order_refund_add) | **POST** /order.refund.add.json | order.refund.add
[**order_return_add**](OrderApi.md#order_return_add) | **POST** /order.return.add.json | order.return.add
[**order_return_delete**](OrderApi.md#order_return_delete) | **DELETE** /order.return.delete.json | order.return.delete
[**order_return_update**](OrderApi.md#order_return_update) | **PUT** /order.return.update.json | order.return.update
[**order_shipment_add**](OrderApi.md#order_shipment_add) | **POST** /order.shipment.add.json | order.shipment.add
[**order_shipment_add_batch**](OrderApi.md#order_shipment_add_batch) | **POST** /order.shipment.add.batch.json | order.shipment.add.batch
[**order_shipment_delete**](OrderApi.md#order_shipment_delete) | **DELETE** /order.shipment.delete.json | order.shipment.delete
[**order_shipment_info**](OrderApi.md#order_shipment_info) | **GET** /order.shipment.info.json | order.shipment.info
[**order_shipment_list**](OrderApi.md#order_shipment_list) | **GET** /order.shipment.list.json | order.shipment.list
[**order_shipment_tracking_add**](OrderApi.md#order_shipment_tracking_add) | **POST** /order.shipment.tracking.add.json | order.shipment.tracking.add
[**order_shipment_update**](OrderApi.md#order_shipment_update) | **PUT** /order.shipment.update.json | order.shipment.update
[**order_status_list**](OrderApi.md#order_status_list) | **GET** /order.status.list.json | order.status.list
[**order_transaction_list**](OrderApi.md#order_transaction_list) | **GET** /order.transaction.list.json | order.transaction.list
[**order_update**](OrderApi.md#order_update) | **PUT** /order.update.json | order.update



## order_abandoned_list

> models::ModelResponseOrderAbandonedList order_abandoned_list(start, count, page_cursor, customer_id, customer_email, store_id, created_from, created_to, modified_from, modified_to, skip_empty_email, response_fields, params, exclude)
order.abandoned.list

Get list of orders that were left by customers before completing the order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**customer_id** | Option<**String**> | Retrieves orders specified by customer id |  |
**customer_email** | Option<**String**> | Retrieves orders specified by customer email |  |
**store_id** | Option<**String**> | Store Id |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**skip_empty_email** | Option<**bool**> | Filter empty emails |  |[default to false]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to customer,totals,items]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseOrderAbandonedList**](Model_Response_Order_Abandoned_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_add

> models::OrderAdd200Response order_add(order_add)
order.add

Add a new order to the cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_add** | [**OrderAdd**](OrderAdd.md) |  | [required] |

### Return type

[**models::OrderAdd200Response**](OrderAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_calculate

> models::OrderCalculate200Response order_calculate(order_calculate)
order.calculate

<p>Calculates the total cost of an order for a given customer and a set of products, as well as the available shipping methods based on the specified address. The calculation takes into account store product prices, discounts, taxes, shipping costs, and other store settings. The result includes a detailed breakdown of the final order cost by its components.</p> <p>Note that the final totals, taxes, and other amounts must include the corresponding values for the selected shipping method.</p><p>The result of this method can be used when creating an order using the <strong>order.add</strong> method.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_calculate** | [**OrderCalculate**](OrderCalculate.md) |  | [required] |

### Return type

[**models::OrderCalculate200Response**](OrderCalculate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_count

> models::OrderCount200Response order_count(order_ids, ids, customer_id, store_id, customer_email, order_status, order_status_ids, ebay_order_status, financial_status, financial_status_ids, fulfillment_channel, fulfillment_status, shipping_method, delivery_method, tags, ship_node_type, created_from, created_to, modified_from, modified_to)
order.count

Count orders in store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_ids** | Option<**String**> | Counts orders specified by order ids |  |
**ids** | Option<**String**> | Counts orders specified by ids |  |
**customer_id** | Option<**String**> | Counts orders quantity specified by customer id |  |
**store_id** | Option<**String**> | Counts orders quantity specified by store id |  |
**customer_email** | Option<**String**> | Counts orders quantity specified by customer email |  |
**order_status** | Option<**String**> | Counts orders quantity specified by order status |  |
**order_status_ids** | Option<[**Vec<String>**](String.md)> | Retrieves orders specified by order statuses |  |
**ebay_order_status** | Option<**String**> | Counts orders quantity specified by order status |  |
**financial_status** | Option<**String**> | Counts orders quantity specified by financial status |  |
**financial_status_ids** | Option<[**Vec<String>**](String.md)> | Retrieves orders count specified by financial status ids |  |
**fulfillment_channel** | Option<**String**> | Retrieves order with a fulfillment channel |  |
**fulfillment_status** | Option<**String**> | Create order with fulfillment status |  |
**shipping_method** | Option<**String**> | Retrieve entities according to shipping method |  |
**delivery_method** | Option<**String**> | Retrieves order with delivery method |  |
**tags** | Option<**String**> | Order tags |  |
**ship_node_type** | Option<**String**> | Retrieves order with ship node type |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |

### Return type

[**models::OrderCount200Response**](OrderCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_financial_status_list

> models::OrderFinancialStatusList200Response order_financial_status_list()
order.financial_status.list

Retrieve list of financial statuses

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrderFinancialStatusList200Response**](OrderFinancialStatusList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_fulfillment_status_list

> models::OrderFulfillmentStatusList200Response order_fulfillment_status_list(action)
order.fulfillment_status.list

Retrieve list of fulfillment statuses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<**String**> | Available statuses for the specified action. |  |

### Return type

[**models::OrderFulfillmentStatusList200Response**](OrderFulfillmentStatusList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_info

> models::OrderInfo200Response order_info(id, order_id, store_id, params, response_fields, exclude, enable_cache, use_latest_api_version, rounding_precision, allow_user_defined_order_statuses)
order.info

Info about a specific order by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Retrieves order info specified by id |  |
**order_id** | Option<**String**> | Retrieves order’s info specified by order id |  |
**store_id** | Option<**String**> | Defines store id where the order should be found |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to order_id,customer,totals,address,items,bundles,status]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**enable_cache** | Option<**bool**> | If the value is 'true' and order exist in our cache, we will return order.info response from cache |  |[default to false]
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version |  |[default to false]
**rounding_precision** | Option<**i32**> | <p>Specifies the rounding precision for fractional numeric values (such as prices, taxes, and weights).</p> <p>Supported values range from <b>1</b> to <b>6</b>.</p> <p>The default rounding precision may vary depending on the platform. You can retrieve the default value using the <strong>cart.info</strong> method in the <code>default_rounding_precision</code> field. </p><p>Values are rounded to the nearest number at the specified precision. Fractions of .5 or higher are rounded up, while fractions lower than .5 are rounded down.</p> |  |
**allow_user_defined_order_statuses** | Option<**bool**> | Indicates whether custom (user-defined) order statuses should be included in the response. |  |[default to false]

### Return type

[**models::OrderInfo200Response**](OrderInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_list

> models::ModelResponseOrderList order_list(start, count, page_cursor, ids, order_ids, since_id, store_id, customer_id, customer_email, basket_id, currency_id, phone, order_status, order_status_ids, ebay_order_status, financial_status, financial_status_ids, fulfillment_status, return_status, fulfillment_channel, shipping_method, skip_order_ids, is_deleted, shipping_country_iso3, delivery_method, ship_node_type, created_to, created_from, modified_to, modified_from, tags, sort_by, sort_direction, params, response_fields, exclude, enable_cache, use_latest_api_version, rounding_precision, allow_user_defined_order_statuses)
order.list

Get list of orders from store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve orders via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**ids** | Option<**String**> | Retrieves orders specified by ids |  |
**order_ids** | Option<**String**> | Retrieves orders specified by order ids |  |
**since_id** | Option<**String**> | Retrieve entities starting from the specified id. |  |
**store_id** | Option<**String**> | Store Id |  |
**customer_id** | Option<**String**> | Retrieves orders specified by customer id |  |
**customer_email** | Option<**String**> | Retrieves orders specified by customer email |  |
**basket_id** | Option<**String**> | Retrieves order’s info specified by basket id. |  |
**currency_id** | Option<**String**> | Currency Id |  |
**phone** | Option<**String**> | Filter orders by customer's phone number |  |
**order_status** | Option<**String**> | Retrieves orders specified by order status |  |
**order_status_ids** | Option<[**Vec<String>**](String.md)> | Retrieves orders specified by order statuses |  |
**ebay_order_status** | Option<**String**> | Retrieves orders specified by order status |  |
**financial_status** | Option<**String**> | Retrieves orders specified by financial status |  |
**financial_status_ids** | Option<[**Vec<String>**](String.md)> | Retrieves orders specified by financial status ids |  |
**fulfillment_status** | Option<**String**> | Create order with fulfillment status |  |
**return_status** | Option<**String**> | Retrieves orders specified by return status |  |
**fulfillment_channel** | Option<**String**> | Retrieves order with a fulfillment channel |  |
**shipping_method** | Option<**String**> | Retrieve entities according to shipping method |  |
**skip_order_ids** | Option<**String**> | Skipped orders by ids |  |
**is_deleted** | Option<**bool**> | Filter deleted orders |  |
**shipping_country_iso3** | Option<**String**> | Retrieve entities according to shipping country |  |
**delivery_method** | Option<**String**> | Retrieves order with delivery method |  |
**ship_node_type** | Option<**String**> | Retrieves order with ship node type |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**tags** | Option<**String**> | Order tags |  |
**sort_by** | Option<**String**> | Set field to sort by |  |[default to order_id]
**sort_direction** | Option<**String**> | Set sorting direction |  |[default to asc]
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to order_id,customer,totals,address,items,bundles,status]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**enable_cache** | Option<**bool**> | If the value is 'true', we will cache orders for a 15 minutes in order to increase speed and reduce requests throttling for some methods and shoping platforms (for example order.shipment.add) |  |[default to false]
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version |  |[default to false]
**rounding_precision** | Option<**i32**> | <p>Specifies the rounding precision for fractional numeric values (such as prices, taxes, and weights).</p> <p>Supported values range from <b>1</b> to <b>6</b>.</p> <p>The default rounding precision may vary depending on the platform. You can retrieve the default value using the <strong>cart.info</strong> method in the <code>default_rounding_precision</code> field. </p><p>Values are rounded to the nearest number at the specified precision. Fractions of .5 or higher are rounded up, while fractions lower than .5 are rounded down.</p> |  |
**allow_user_defined_order_statuses** | Option<**bool**> | Indicates whether custom (user-defined) order statuses should be included in the response. |  |[default to false]

### Return type

[**models::ModelResponseOrderList**](Model_Response_Order_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_preestimate_shipping_list

> models::ModelResponseOrderPreestimateShippingList order_preestimate_shipping_list(order_preestimate_shipping_list)
order.preestimate_shipping.list

Retrieve list of order preestimated shipping methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_preestimate_shipping_list** | [**OrderPreestimateShippingList**](OrderPreestimateShippingList.md) |  | [required] |

### Return type

[**models::ModelResponseOrderPreestimateShippingList**](Model_Response_Order_PreestimateShipping_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_refund_add

> models::OrderRefundAdd200Response order_refund_add(order_refund_add)
order.refund.add

Add a refund to the order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_refund_add** | [**OrderRefundAdd**](OrderRefundAdd.md) |  | [required] |

### Return type

[**models::OrderRefundAdd200Response**](OrderRefundAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_return_add

> models::OrderReturnAdd200Response order_return_add(order_return_add)
order.return.add

Create new return request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_return_add** | [**OrderReturnAdd**](OrderReturnAdd.md) |  | [required] |

### Return type

[**models::OrderReturnAdd200Response**](OrderReturnAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_return_delete

> models::AttributeValueDelete200Response order_return_delete(return_id, order_id, store_id)
order.return.delete

Delete return.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_id** | **String** | Return ID | [required] |
**order_id** | **String** | Defines the order id | [required] |
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::AttributeValueDelete200Response**](AttributeValueDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_return_update

> models::AccountConfigUpdate200Response order_return_update(order_return_update)
order.return.update

Update order's shipment information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_return_update** | [**OrderReturnUpdate**](OrderReturnUpdate.md) |  | [required] |

### Return type

[**models::AccountConfigUpdate200Response**](AccountConfigUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_shipment_add

> models::OrderShipmentAdd200Response order_shipment_add(order_shipment_add)
order.shipment.add

Add a shipment to the order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_shipment_add** | [**OrderShipmentAdd**](OrderShipmentAdd.md) |  | [required] |

### Return type

[**models::OrderShipmentAdd200Response**](OrderShipmentAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_shipment_add_batch

> models::CategoryAddBatch200Response order_shipment_add_batch(order_shipment_add_batch)
order.shipment.add.batch

Add a shipments to the orders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_shipment_add_batch** | [**OrderShipmentAddBatch**](OrderShipmentAddBatch.md) |  | [required] |

### Return type

[**models::CategoryAddBatch200Response**](CategoryAddBatch_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_shipment_delete

> models::OrderShipmentDelete200Response order_shipment_delete(shipment_id, order_id, store_id)
order.shipment.delete

Delete order's shipment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** | Shipment id indicates the number of delivery | [required] |
**order_id** | **String** | Defines the order for which the shipment will be deleted | [required] |
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::OrderShipmentDelete200Response**](OrderShipmentDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_shipment_info

> models::OrderShipmentInfo200Response order_shipment_info(id, order_id, start, store_id, response_fields, params, exclude)
order.shipment.info

Get information of shipment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**order_id** | **String** | Defines the order id | [required] |
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**store_id** | Option<**String**> | Store Id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,order_id,items,tracking_numbers]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::OrderShipmentInfo200Response**](OrderShipmentInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_shipment_list

> models::ModelResponseOrderShipmentList order_shipment_list(order_id, start, count, page_cursor, store_id, created_from, created_to, modified_from, modified_to, response_fields, params, exclude)
order.shipment.list

Get list of shipments per order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Retrieves shipments specified by order id | [required] |
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**store_id** | Option<**String**> | Store Id |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,order_id,items,tracking_numbers]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseOrderShipmentList**](Model_Response_Order_Shipment_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_shipment_tracking_add

> models::OrderShipmentTrackingAdd200Response order_shipment_tracking_add(order_shipment_tracking_add)
order.shipment.tracking.add

Add order shipment's tracking info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_shipment_tracking_add** | [**OrderShipmentTrackingAdd**](OrderShipmentTrackingAdd.md) |  | [required] |

### Return type

[**models::OrderShipmentTrackingAdd200Response**](OrderShipmentTrackingAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_shipment_update

> models::AccountConfigUpdate200Response order_shipment_update(order_shipment_update)
order.shipment.update

Update order's shipment information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_shipment_update** | [**OrderShipmentUpdate**](OrderShipmentUpdate.md) |  | [required] |

### Return type

[**models::AccountConfigUpdate200Response**](AccountConfigUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_status_list

> models::ModelResponseOrderStatusList order_status_list(store_id, action, allow_user_defined_order_statuses, response_fields)
order.status.list

Retrieve list of statuses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**String**> | Store Id |  |
**action** | Option<**String**> | Available statuses for the specified action. |  |
**allow_user_defined_order_statuses** | Option<**bool**> | Indicates whether custom (user-defined) order statuses should be included in the response. |  |[default to false]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |

### Return type

[**models::ModelResponseOrderStatusList**](Model_Response_Order_Status_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_transaction_list

> models::ModelResponseOrderTransactionList order_transaction_list(order_ids, count, page_cursor, store_id, params, response_fields, exclude)
order.transaction.list

Retrieve list of order transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_ids** | **String** | Retrieves order transactions specified by order ids | [required] |
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**store_id** | Option<**String**> | Store Id |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,order_id,amount,description]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseOrderTransactionList**](Model_Response_Order_Transaction_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_update

> models::AccountConfigUpdate200Response order_update(order_id, store_id, order_status, financial_status, fulfillment_status, cancellation_reason, order_payment_method, comment, admin_comment, admin_private_comment, invoice_admin_comment, date_modified, date_finished, send_notifications, create_invoice, origin, tags, idempotency_key)
order.update

Update existing order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Defines the orders specified by order id | [required] |
**store_id** | Option<**String**> | Defines store id where the order should be found |  |
**order_status** | Option<**String**> | Defines new order's status |  |
**financial_status** | Option<**String**> | Update order financial status to specified |  |
**fulfillment_status** | Option<**String**> | Create order with fulfillment status |  |
**cancellation_reason** | Option<**String**> | Defines the cancellation reason when the order will be canceled |  |
**order_payment_method** | Option<**String**> | Defines order payment method.<br/>Setting order_payment_method on Shopify will also change financial_status field value to 'paid' |  |
**comment** | Option<**String**> | Specifies order comment |  |
**admin_comment** | Option<**String**> | Specifies admin's order comment |  |
**admin_private_comment** | Option<**String**> | Specifies private admin's order comment |  |
**invoice_admin_comment** | Option<**String**> | Specifies admin's order invoice comment |  |
**date_modified** | Option<**String**> | Specifies order's  modification date |  |
**date_finished** | Option<**String**> | Specifies order's  finished date |  |
**send_notifications** | Option<**bool**> | Send notifications to customer after order was created |  |[default to false]
**create_invoice** | Option<**bool**> | Determines whether an invoice should be created if it has not already been created |  |
**origin** | Option<**String**> | The source of the order |  |
**tags** | Option<**String**> | Order tags |  |
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> |  |

### Return type

[**models::AccountConfigUpdate200Response**](AccountConfigUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


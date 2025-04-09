# \OrderApi

All URIs are relative to *https://api.api2cart.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**order_abandoned_list**](OrderApi.md#order_abandoned_list) | **GET** /order.abandoned.list.json | order.abandoned.list
[**order_add**](OrderApi.md#order_add) | **POST** /order.add.json | order.add
[**order_count**](OrderApi.md#order_count) | **GET** /order.count.json | order.count
[**order_financial_status_list**](OrderApi.md#order_financial_status_list) | **GET** /order.financial_status.list.json | order.financial_status.list
[**order_find**](OrderApi.md#order_find) | **GET** /order.find.json | order.find
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

> models::ModelResponseOrderAbandonedList order_abandoned_list(customer_id, customer_email, created_to, created_from, modified_to, modified_from, skip_empty_email, store_id, page_cursor, count, start, params, response_fields, exclude)
order.abandoned.list

Get list of orders that were left by customers before completing the order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | Retrieves orders specified by customer id |  |
**customer_email** | Option<**String**> | Retrieves orders specified by customer email |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**skip_empty_email** | Option<**bool**> | Filter empty emails |  |[default to false]
**store_id** | Option<**String**> | Store Id |  |
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to customer,totals,items]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
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


## order_count

> models::OrderCount200Response order_count(customer_id, customer_email, order_status, order_status_ids, created_to, created_from, modified_to, modified_from, store_id, ids, order_ids, ebay_order_status, financial_status, financial_status_ids, fulfillment_channel, fulfillment_status, shipping_method, delivery_method, tags, ship_node_type)
order.count

Count orders in store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | Counts orders quantity specified by customer id |  |
**customer_email** | Option<**String**> | Counts orders quantity specified by customer email |  |
**order_status** | Option<**String**> | Counts orders quantity specified by order status |  |
**order_status_ids** | Option<[**Vec<String>**](String.md)> | Retrieves orders specified by order statuses |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**store_id** | Option<**String**> | Counts orders quantity specified by store id |  |
**ids** | Option<**String**> | Counts orders specified by ids |  |
**order_ids** | Option<**String**> | Counts orders specified by order ids |  |
**ebay_order_status** | Option<**String**> | Counts orders quantity specified by order status |  |
**financial_status** | Option<**String**> | Counts orders quantity specified by financial status |  |
**financial_status_ids** | Option<[**Vec<String>**](String.md)> | Retrieves orders count specified by financial status ids |  |
**fulfillment_channel** | Option<**String**> | Retrieves order with a fulfillment channel |  |
**fulfillment_status** | Option<**String**> | Create order with fulfillment status |  |
**shipping_method** | Option<**String**> | Retrieve entities according to shipping method |  |
**delivery_method** | Option<**String**> | Retrieves order with delivery method |  |
**tags** | Option<**String**> | Order tags |  |
**ship_node_type** | Option<**String**> | Retrieves order with ship node type |  |

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


## order_find

> models::OrderFind200Response order_find(customer_id, customer_email, order_status, start, count, params, exclude, created_to, created_from, modified_to, modified_from, financial_status)
order.find

This method is deprecated and won't be supported in the future. Please use \"order.list\" instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | Retrieves orders specified by customer id |  |
**customer_email** | Option<**String**> | Retrieves orders specified by customer email |  |
**order_status** | Option<**String**> | Retrieves orders specified by order status |  |
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to order_id,customer,totals,address,items,bundles,status]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**financial_status** | Option<**String**> | Retrieves orders specified by financial status |  |

### Return type

[**models::OrderFind200Response**](OrderFind_200_response.md)

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

> models::OrderInfo200Response order_info(order_id, id, params, response_fields, exclude, store_id, enable_cache, use_latest_api_version)
order.info

Info about a specific order by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | Option<**String**> | Retrieves order’s info specified by order id |  |
**id** | Option<**String**> | Retrieves order info specified by id |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to order_id,customer,totals,address,items,bundles,status]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**store_id** | Option<**String**> | Defines store id where the order should be found |  |
**enable_cache** | Option<**bool**> | If the value is 'true' and order exist in our cache, we will return order.info response from cache |  |[default to false]
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version |  |[default to false]

### Return type

[**models::OrderInfo200Response**](OrderInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_list

> models::ModelResponseOrderList order_list(customer_id, customer_email, phone, order_status, order_status_ids, start, count, page_cursor, sort_by, sort_direction, params, response_fields, exclude, created_to, created_from, modified_to, modified_from, store_id, ids, order_ids, ebay_order_status, basket_id, financial_status, financial_status_ids, fulfillment_status, fulfillment_channel, shipping_method, skip_order_ids, since_id, is_deleted, shipping_country_iso3, enable_cache, delivery_method, tags, ship_node_type, currency_id, return_status, use_latest_api_version)
order.list

Get list of orders from store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | Retrieves orders specified by customer id |  |
**customer_email** | Option<**String**> | Retrieves orders specified by customer email |  |
**phone** | Option<**String**> | Filter orders by customer's phone number |  |
**order_status** | Option<**String**> | Retrieves orders specified by order status |  |
**order_status_ids** | Option<[**Vec<String>**](String.md)> | Retrieves orders specified by order statuses |  |
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve orders via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**sort_by** | Option<**String**> | Set field to sort by |  |[default to order_id]
**sort_direction** | Option<**String**> | Set sorting direction |  |[default to asc]
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to order_id,customer,totals,address,items,bundles,status]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**store_id** | Option<**String**> | Store Id |  |
**ids** | Option<**String**> | Retrieves orders specified by ids |  |
**order_ids** | Option<**String**> | Retrieves orders specified by order ids |  |
**ebay_order_status** | Option<**String**> | Retrieves orders specified by order status |  |
**basket_id** | Option<**String**> | Retrieves order’s info specified by basket id. |  |
**financial_status** | Option<**String**> | Retrieves orders specified by financial status |  |
**financial_status_ids** | Option<[**Vec<String>**](String.md)> | Retrieves orders specified by financial status ids |  |
**fulfillment_status** | Option<**String**> | Create order with fulfillment status |  |
**fulfillment_channel** | Option<**String**> | Retrieves order with a fulfillment channel |  |
**shipping_method** | Option<**String**> | Retrieve entities according to shipping method |  |
**skip_order_ids** | Option<**String**> | Skipped orders by ids |  |
**since_id** | Option<**String**> | Retrieve entities starting from the specified id. |  |
**is_deleted** | Option<**bool**> | Filter deleted orders |  |
**shipping_country_iso3** | Option<**String**> | Retrieve entities according to shipping country |  |
**enable_cache** | Option<**bool**> | If the value is 'true', we will cache orders for a 15 minutes in order to increase speed and reduce requests throttling for some methods and shoping platforms (for example order.shipment.add) |  |[default to false]
**delivery_method** | Option<**String**> | Retrieves order with delivery method |  |
**tags** | Option<**String**> | Order tags |  |
**ship_node_type** | Option<**String**> | Retrieves order with ship node type |  |
**currency_id** | Option<**String**> | Currency Id |  |
**return_status** | Option<**String**> | Retrieves orders specified by return status |  |
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version |  |[default to false]

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

> models::OrderShipmentInfo200Response order_shipment_info(id, order_id, start, params, response_fields, exclude, store_id)
order.shipment.info

Get information of shipment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**order_id** | **String** | Defines the order id | [required] |
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,order_id,items,tracking_numbers]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::OrderShipmentInfo200Response**](OrderShipmentInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_shipment_list

> models::ModelResponseOrderShipmentList order_shipment_list(order_id, page_cursor, start, count, params, response_fields, exclude, created_from, created_to, modified_from, modified_to, store_id)
order.shipment.list

Get list of shipments by orders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Retrieves shipments specified by order id | [required] |
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,order_id,items,tracking_numbers]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**store_id** | Option<**String**> | Store Id |  |

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

> models::ModelResponseOrderStatusList order_status_list(store_id, action, response_fields)
order.status.list

Retrieve list of statuses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**String**> | Store Id |  |
**action** | Option<**String**> | Available statuses for the specified action. |  |
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

> models::ModelResponseOrderTransactionList order_transaction_list(order_ids, count, store_id, params, response_fields, exclude, page_cursor)
order.transaction.list

Retrieve list of order transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_ids** | **String** | Retrieves order transactions specified by order ids | [required] |
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**store_id** | Option<**String**> | Store Id |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,order_id,amount,description]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |

### Return type

[**models::ModelResponseOrderTransactionList**](Model_Response_Order_Transaction_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_update

> models::AccountConfigUpdate200Response order_update(order_id, store_id, order_status, cancellation_reason, comment, admin_comment, admin_private_comment, date_modified, date_finished, financial_status, fulfillment_status, order_payment_method, send_notifications, origin, create_invoice, invoice_admin_comment)
order.update

Update existing order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Defines the orders specified by order id | [required] |
**store_id** | Option<**String**> | Defines store id where the order should be found |  |
**order_status** | Option<**String**> | Defines new order's status |  |
**cancellation_reason** | Option<**String**> | Defines the cancellation reason when the order will be canceled |  |
**comment** | Option<**String**> | Specifies order comment |  |
**admin_comment** | Option<**String**> | Specifies admin's order comment |  |
**admin_private_comment** | Option<**String**> | Specifies private admin's order comment |  |
**date_modified** | Option<**String**> | Specifies order's  modification date |  |
**date_finished** | Option<**String**> | Specifies order's  finished date |  |
**financial_status** | Option<**String**> | Update order financial status to specified |  |
**fulfillment_status** | Option<**String**> | Create order with fulfillment status |  |
**order_payment_method** | Option<**String**> | Defines order payment method.<br/>Setting order_payment_method on Shopify will also change financial_status field value to 'paid' |  |
**send_notifications** | Option<**bool**> | Send notifications to customer after order was created |  |[default to false]
**origin** | Option<**String**> | The source of the order |  |
**create_invoice** | Option<**bool**> | Determines whether an invoice should be created if it has not already been created |  |
**invoice_admin_comment** | Option<**String**> | Specifies admin's order invoice comment |  |

### Return type

[**models::AccountConfigUpdate200Response**](AccountConfigUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


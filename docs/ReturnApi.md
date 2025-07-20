# \ReturnApi

All URIs are relative to *https://api.api2cart.local.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**return_action_list**](ReturnApi.md#return_action_list) | **GET** /return.action.list.json | return.action.list
[**return_count**](ReturnApi.md#return_count) | **GET** /return.count.json | return.count
[**return_info**](ReturnApi.md#return_info) | **GET** /return.info.json | return.info
[**return_list**](ReturnApi.md#return_list) | **GET** /return.list.json | return.list
[**return_reason_list**](ReturnApi.md#return_reason_list) | **GET** /return.reason.list.json | return.reason.list
[**return_status_list**](ReturnApi.md#return_status_list) | **GET** /return.status.list.json | return.status.list



## return_action_list

> models::ReturnActionList200Response return_action_list()
return.action.list

Retrieve list of return actions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReturnActionList200Response**](ReturnActionList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## return_count

> models::ReturnCount200Response return_count(order_ids, customer_id, store_id, status, return_type, created_from, created_to, modified_from, modified_to, report_request_id, disable_report_cache)
return.count

Count returns in store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_ids** | Option<**String**> | Counts return requests specified by order ids |  |
**customer_id** | Option<**String**> | Counts return requests quantity specified by customer id |  |
**store_id** | Option<**String**> | Store Id |  |
**status** | Option<**String**> | Defines status |  |
**return_type** | Option<**String**> | Retrieves returns specified by return type |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**report_request_id** | Option<**String**> | Report request id |  |
**disable_report_cache** | Option<**bool**> | Disable report cache for current request |  |[default to false]

### Return type

[**models::ReturnCount200Response**](ReturnCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## return_info

> models::ReturnInfo200Response return_info(id, order_id, store_id, response_fields, params, exclude)
return.info

Retrieve return information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**order_id** | Option<**String**> | Defines the order id |  |
**store_id** | Option<**String**> | Store Id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,order_products]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ReturnInfo200Response**](ReturnInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## return_list

> models::ModelResponseReturnList return_list(start, count, page_cursor, order_id, order_ids, customer_id, store_id, status, return_type, created_from, created_to, modified_from, modified_to, response_fields, params, exclude, report_request_id, disable_report_cache)
return.list

Get list of return requests from store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**order_id** | Option<**String**> | Defines the order id |  |
**order_ids** | Option<**String**> | Retrieves return requests specified by order ids |  |
**customer_id** | Option<**String**> | Retrieves return requests specified by customer id |  |
**store_id** | Option<**String**> | Store Id |  |
**status** | Option<**String**> | Defines status |  |
**return_type** | Option<**String**> | Retrieves returns specified by return type |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,order_products]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**report_request_id** | Option<**String**> | Report request id |  |
**disable_report_cache** | Option<**bool**> | Disable report cache for current request |  |[default to false]

### Return type

[**models::ModelResponseReturnList**](Model_Response_Return_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## return_reason_list

> models::ReturnReasonList200Response return_reason_list(store_id)
return.reason.list

Retrieve list of return reasons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::ReturnReasonList200Response**](ReturnReasonList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## return_status_list

> models::ReturnStatusList200Response return_status_list()
return.status.list

Retrieve list of statuses

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReturnStatusList200Response**](ReturnStatusList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


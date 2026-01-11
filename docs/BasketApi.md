# \BasketApi

All URIs are relative to *https://api.api2cart.local.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basket_info**](BasketApi.md#basket_info) | **GET** /basket.info.json | basket.info
[**basket_item_add**](BasketApi.md#basket_item_add) | **POST** /basket.item.add.json | basket.item.add
[**basket_live_shipping_service_create**](BasketApi.md#basket_live_shipping_service_create) | **POST** /basket.live_shipping_service.create.json | basket.live_shipping_service.create
[**basket_live_shipping_service_delete**](BasketApi.md#basket_live_shipping_service_delete) | **DELETE** /basket.live_shipping_service.delete.json | basket.live_shipping_service.delete
[**basket_live_shipping_service_list**](BasketApi.md#basket_live_shipping_service_list) | **GET** /basket.live_shipping_service.list.json | basket.live_shipping_service.list



## basket_info

> models::BasketInfo200Response basket_info(id, store_id, response_fields, params, exclude)
basket.info

Retrieve basket information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**store_id** | Option<**String**> | Store Id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to force_all]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::BasketInfo200Response**](BasketInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basket_item_add

> models::BasketItemAdd200Response basket_item_add(customer_id, product_id, variant_id, quantity, store_id, idempotency_key)
basket.item.add

Add item to basket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Retrieves orders specified by customer id | [required] |
**product_id** | **String** | Defines id of the product which should be added to the basket | [required] |
**variant_id** | Option<**String**> | Defines product's variants specified by variant id |  |
**quantity** | Option<**f64**> | Defines new items quantity |  |[default to 0]
**store_id** | Option<**String**> | Store Id |  |
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> |  |

### Return type

[**models::BasketItemAdd200Response**](BasketItemAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basket_live_shipping_service_create

> models::BasketLiveShippingServiceCreate200Response basket_live_shipping_service_create(name, callback, store_id, idempotency_key)
basket.live_shipping_service.create

Create live shipping rate service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Shipping Service Name | [required] |
**callback** | **String** | Callback url that returns shipping rates. It should be able to accept POST requests with json data. | [required] |
**store_id** | Option<**String**> | Store Id |  |
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> |  |

### Return type

[**models::BasketLiveShippingServiceCreate200Response**](BasketLiveShippingServiceCreate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basket_live_shipping_service_delete

> models::BasketLiveShippingServiceDelete200Response basket_live_shipping_service_delete(id)
basket.live_shipping_service.delete

Delete live shipping rate service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Entity id | [required] |

### Return type

[**models::BasketLiveShippingServiceDelete200Response**](BasketLiveShippingServiceDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basket_live_shipping_service_list

> models::BasketLiveShippingServiceList200Response basket_live_shipping_service_list(start, count, store_id)
basket.live_shipping_service.list

Retrieve a list of live shipping rate services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::BasketLiveShippingServiceList200Response**](BasketLiveShippingServiceList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


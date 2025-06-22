# \WebhookApi

All URIs are relative to *https://api.api2cart.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhook_count**](WebhookApi.md#webhook_count) | **GET** /webhook.count.json | webhook.count
[**webhook_create**](WebhookApi.md#webhook_create) | **POST** /webhook.create.json | webhook.create
[**webhook_delete**](WebhookApi.md#webhook_delete) | **DELETE** /webhook.delete.json | webhook.delete
[**webhook_events**](WebhookApi.md#webhook_events) | **GET** /webhook.events.json | webhook.events
[**webhook_list**](WebhookApi.md#webhook_list) | **GET** /webhook.list.json | webhook.list
[**webhook_update**](WebhookApi.md#webhook_update) | **PUT** /webhook.update.json | webhook.update



## webhook_count

> models::WebhookCount200Response webhook_count(entity, action, active)
webhook.count

Count registered webhooks on the store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | Option<**String**> | The entity you want to filter webhooks by (e.g. order or product) |  |
**action** | Option<**String**> | The action you want to filter webhooks by (e.g. order or product) |  |
**active** | Option<**bool**> | The webhook status you want to filter webhooks by |  |

### Return type

[**models::WebhookCount200Response**](WebhookCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_create

> models::BasketLiveShippingServiceCreate200Response webhook_create(entity, action, callback, label, fields, response_fields, active, lang_id, store_id)
webhook.create

Create webhook on the store and subscribe to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | **String** | Specify the entity that you want to enable webhooks for (e.g product, order, customer, category) | [required] |
**action** | **String** | Specify what action (event) will trigger the webhook (e.g add, delete, or update) | [required] |
**callback** | Option<**String**> | Callback url that returns shipping rates. It should be able to accept POST requests with json data. |  |
**label** | Option<**String**> | The name you give to the webhook |  |
**fields** | Option<**String**> | Fields the webhook should send |  |[default to force_all]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**active** | Option<**bool**> | Webhook status |  |[default to true]
**lang_id** | Option<**String**> | Language id |  |
**store_id** | Option<**String**> | Defines store id where the webhook should be assigned |  |

### Return type

[**models::BasketLiveShippingServiceCreate200Response**](BasketLiveShippingServiceCreate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_delete

> models::AttributeDelete200Response webhook_delete(id)
webhook.delete

Delete registered webhook on the store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Webhook id | [required] |

### Return type

[**models::AttributeDelete200Response**](AttributeDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_events

> models::WebhookEvents200Response webhook_events()
webhook.events

List all Webhooks that are available on this store.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebhookEvents200Response**](WebhookEvents_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_list

> models::WebhookList200Response webhook_list(start, count, entity, action, active, ids, params)
webhook.list

List registered webhook on the store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**entity** | Option<**String**> | The entity you want to filter webhooks by (e.g. order or product) |  |
**action** | Option<**String**> | The action you want to filter webhooks by (e.g. add, update, or delete) |  |
**active** | Option<**bool**> | The webhook status you want to filter webhooks by |  |
**ids** | Option<**String**> | List of сomma-separated webhook ids |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,entity,action,callback]

### Return type

[**models::WebhookList200Response**](WebhookList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_update

> models::ProductImageUpdate200Response webhook_update(id, callback, label, fields, response_fields, active, lang_id)
webhook.update

Update Webhooks parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Webhook id | [required] |
**callback** | Option<**String**> | Callback url that returns shipping rates. It should be able to accept POST requests with json data. |  |
**label** | Option<**String**> | The name you give to the webhook |  |
**fields** | Option<**String**> | Fields the webhook should send |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**active** | Option<**bool**> | Webhook status |  |
**lang_id** | Option<**String**> | Language id |  |

### Return type

[**models::ProductImageUpdate200Response**](ProductImageUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


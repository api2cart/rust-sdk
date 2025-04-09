# \BridgeApi

All URIs are relative to *https://api.api2cart.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bridge_delete**](BridgeApi.md#bridge_delete) | **POST** /bridge.delete.json | bridge.delete
[**bridge_download**](BridgeApi.md#bridge_download) | **GET** /bridge.download.file | bridge.download
[**bridge_update**](BridgeApi.md#bridge_update) | **POST** /bridge.update.json | bridge.update



## bridge_delete

> models::AttributeValueDelete200Response bridge_delete()
bridge.delete

Delete bridge from the store.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AttributeValueDelete200Response**](AttributeValueDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bridge_download

> std::path::PathBuf bridge_download(whitelabel)
bridge.download

Download bridge for store.</br>Please note that the method would not work if you call it from Swagger UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**whitelabel** | Option<**bool**> | Identifies if there is a necessity to download whitelabel bridge. |  |[default to false]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bridge_update

> models::AttributeUpdate200Response bridge_update()
bridge.update

Update bridge in the store.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AttributeUpdate200Response**](AttributeUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


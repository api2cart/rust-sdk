# \TaxApi

All URIs are relative to *https://api.api2cart.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tax_class_info**](TaxApi.md#tax_class_info) | **GET** /tax.class.info.json | tax.class.info
[**tax_class_list**](TaxApi.md#tax_class_list) | **GET** /tax.class.list.json | tax.class.list



## tax_class_info

> models::ModelResponseTaxClassInfo tax_class_info(tax_class_id, store_id, lang_id, params, response_fields, exclude)
tax.class.info

Use this method to get information about a tax class and its rates. It allows you to calculate the tax percentage for a specific customer's address. This information contains relatively static data that rarely changes, so API2Cart may cache certain data to reduce the load on the store and speed up request execution. We also recommend that you cache the response of this method on your side to save requests. If you need to clear the cache for a specific store, use the cart.validate method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tax_class_id** | **String** | Retrieves taxes specified by class id | [required] |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to tax_class_id,name,avail]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseTaxClassInfo**](Model_Response_Tax_Class_Info.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_class_list

> models::ModelResponseTaxClassList tax_class_list(created_to, created_from, modified_to, modified_from, find_value, find_where, store_id, count, page_cursor, response_fields)
tax.class.list

Get list of tax classes from your store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**find_value** | Option<**String**> | Entity search that is specified by some value |  |
**find_where** | Option<**String**> | Tax class search that is specified by field |  |
**store_id** | Option<**String**> | Store Id |  |
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to {return_code,return_message,pagination,result}]

### Return type

[**models::ModelResponseTaxClassList**](Model_Response_Tax_Class_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


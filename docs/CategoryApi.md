# \CategoryApi

All URIs are relative to *https://api.api2cart.local.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**category_add**](CategoryApi.md#category_add) | **POST** /category.add.json | category.add
[**category_add_batch**](CategoryApi.md#category_add_batch) | **POST** /category.add.batch.json | category.add.batch
[**category_assign**](CategoryApi.md#category_assign) | **POST** /category.assign.json | category.assign
[**category_count**](CategoryApi.md#category_count) | **GET** /category.count.json | category.count
[**category_delete**](CategoryApi.md#category_delete) | **DELETE** /category.delete.json | category.delete
[**category_delete_batch**](CategoryApi.md#category_delete_batch) | **POST** /category.delete.batch.json | category.delete.batch
[**category_find**](CategoryApi.md#category_find) | **GET** /category.find.json | category.find
[**category_image_add**](CategoryApi.md#category_image_add) | **POST** /category.image.add.json | category.image.add
[**category_image_delete**](CategoryApi.md#category_image_delete) | **DELETE** /category.image.delete.json | category.image.delete
[**category_info**](CategoryApi.md#category_info) | **GET** /category.info.json | category.info
[**category_list**](CategoryApi.md#category_list) | **GET** /category.list.json | category.list
[**category_unassign**](CategoryApi.md#category_unassign) | **POST** /category.unassign.json | category.unassign
[**category_update**](CategoryApi.md#category_update) | **PUT** /category.update.json | category.update



## category_add

> models::CategoryAdd200Response category_add(name, description, short_description, parent_id, avail, created_time, modified_time, sort_order, meta_title, meta_description, meta_keywords, seo_url, store_id, stores_ids, lang_id, idempotency_key)
category.add

Add new category in store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Defines category's name that has to be added | [required] |
**description** | Option<**String**> | Defines category's description |  |
**short_description** | Option<**String**> | Defines short description |  |
**parent_id** | Option<**String**> | Adds categories specified by parent id |  |
**avail** | Option<**bool**> | Defines category's visibility status |  |[default to true]
**created_time** | Option<**String**> | Entity's date creation |  |
**modified_time** | Option<**String**> | Entity's date modification |  |
**sort_order** | Option<**i32**> | Sort number in the list |  |[default to 0]
**meta_title** | Option<**String**> | Defines unique meta title for each entity |  |
**meta_description** | Option<**String**> | Defines unique meta description of a entity |  |
**meta_keywords** | Option<**String**> | Defines unique meta keywords for each entity |  |
**seo_url** | Option<**String**> | Defines unique category's URL for SEO |  |
**store_id** | Option<**String**> | Store Id |  |
**stores_ids** | Option<**String**> | Create category in the stores that is specified by comma-separated stores' id |  |
**lang_id** | Option<**String**> | Language id |  |
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> |  |

### Return type

[**models::CategoryAdd200Response**](CategoryAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_add_batch

> models::CategoryAddBatch200Response category_add_batch(category_add_batch)
category.add.batch

Add new categories to the store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_add_batch** | [**CategoryAddBatch**](CategoryAddBatch.md) |  | [required] |

### Return type

[**models::CategoryAddBatch200Response**](CategoryAddBatch_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_assign

> models::CategoryAssign200Response category_assign(category_id, product_id, store_id, idempotency_key)
category.assign

Assign category to product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | Defines category assign, specified by category id | [required] |
**product_id** | **String** | Defines category assign to the product, specified by product id | [required] |
**store_id** | Option<**String**> | Store Id |  |
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> |  |

### Return type

[**models::CategoryAssign200Response**](CategoryAssign_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_count

> models::CategoryCount200Response category_count(parent_id, store_id, lang_id, avail, created_from, created_to, modified_from, modified_to, product_type, find_value, find_where, report_request_id, disable_report_cache)
category.count

Count categories in store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_id** | Option<**String**> | Counts categories specified by parent id |  |
**store_id** | Option<**String**> | Counts category specified by store id |  |
**lang_id** | Option<**String**> | Counts category specified by language id |  |
**avail** | Option<**bool**> | Defines category's visibility status |  |[default to true]
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**product_type** | Option<**String**> | A categorization for the product |  |
**find_value** | Option<**String**> | Entity search that is specified by some value |  |
**find_where** | Option<**String**> | Counts categories that are searched specified by field |  |
**report_request_id** | Option<**String**> | Report request id |  |
**disable_report_cache** | Option<**bool**> | Disable report cache for current request |  |[default to false]

### Return type

[**models::CategoryCount200Response**](CategoryCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_delete

> models::CategoryDelete200Response category_delete(id, store_id)
category.delete

Delete category in store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Defines category removal, specified by category id | [required] |
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::CategoryDelete200Response**](CategoryDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_delete_batch

> models::CategoryAddBatch200Response category_delete_batch(category_delete_batch)
category.delete.batch

Delete categories from the store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_delete_batch** | [**CategoryDeleteBatch**](CategoryDeleteBatch.md) |  | [required] |

### Return type

[**models::CategoryAddBatch200Response**](CategoryAddBatch_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_find

> models::CategoryFind200Response category_find(find_value, find_where, find_params, store_id, lang_id)
category.find

Search category in store. \"Laptop\" is specified here by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**find_value** | **String** | Entity search that is specified by some value | [required] |
**find_where** | Option<**String**> | Entity search that is specified by the comma-separated unique fields |  |[default to name]
**find_params** | Option<**String**> | Entity search that is specified by comma-separated parameters |  |[default to whole_words]
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |

### Return type

[**models::CategoryFind200Response**](CategoryFind_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_image_add

> models::CategoryImageAdd200Response category_image_add(category_id, image_name, url, r#type, store_id, label, mime, position, apply_to_translations, idempotency_key)
category.image.add

Add image to category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | Defines category id where the image should be added | [required] |
**image_name** | **String** | Defines image's name | [required] |
**url** | **String** | Defines URL of the image that has to be added | [required] |
**r#type** | **String** | Defines image's types that are specified by comma-separated list | [required] |
**store_id** | Option<**String**> | Store Id |  |
**label** | Option<**String**> | Defines alternative text that has to be attached to the picture |  |
**mime** | Option<**String**> | Mime type of image http://en.wikipedia.org/wiki/Internet_media_type. |  |
**position** | Option<**i32**> | Defines image’s position in the list |  |[default to 0]
**apply_to_translations** | Option<**bool**> | Defines whether to add image to all category translations |  |[default to true]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> |  |

### Return type

[**models::CategoryImageAdd200Response**](CategoryImageAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_image_delete

> models::AttributeDelete200Response category_image_delete(category_id, image_id, store_id, apply_to_translations)
category.image.delete

Delete image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | Defines category id where the image should be deleted | [required] |
**image_id** | **String** | Define image id | [required] |
**store_id** | Option<**String**> | Store Id |  |
**apply_to_translations** | Option<**bool**> | Defines whether to delete image from all category translations |  |[default to true]

### Return type

[**models::AttributeDelete200Response**](AttributeDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_info

> models::CategoryInfo200Response category_info(id, store_id, lang_id, schema_type, response_fields, params, exclude, report_request_id, disable_report_cache, use_latest_api_version)
category.info

Get category info about category ID*** or specify other category ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Retrieves category's info specified by category id | [required] |
**store_id** | Option<**String**> | Retrieves category info  specified by store id |  |
**lang_id** | Option<**String**> | Retrieves category info  specified by language id |  |
**schema_type** | Option<**String**> | The name of the requirements set for the provided schema. |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,parent_id,name,description]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**report_request_id** | Option<**String**> | Report request id |  |
**disable_report_cache** | Option<**bool**> | Disable report cache for current request |  |[default to false]
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version |  |[default to false]

### Return type

[**models::CategoryInfo200Response**](CategoryInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_list

> models::ModelResponseCategoryList category_list(start, count, page_cursor, store_id, lang_id, parent_id, avail, product_type, created_from, created_to, modified_from, modified_to, find_value, find_where, response_fields, params, exclude, report_request_id, disable_report_cache, disable_cache, use_latest_api_version)
category.list

Get list of categories from store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**store_id** | Option<**String**> | Retrieves categories specified by store id |  |
**lang_id** | Option<**String**> | Retrieves categorys specified by language id |  |
**parent_id** | Option<**String**> | Retrieves categories specified by parent id |  |
**avail** | Option<**bool**> | Defines category's visibility status |  |[default to true]
**product_type** | Option<**String**> | A categorization for the product |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**find_value** | Option<**String**> | Entity search that is specified by some value |  |
**find_where** | Option<**String**> | Category search that is specified by field |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,parent_id,name,description]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**report_request_id** | Option<**String**> | Report request id |  |
**disable_report_cache** | Option<**bool**> | Disable report cache for current request |  |[default to false]
**disable_cache** | Option<**bool**> | Disable cache for current request |  |[default to false]
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version |  |[default to false]

### Return type

[**models::ModelResponseCategoryList**](Model_Response_Category_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_unassign

> models::CategoryAssign200Response category_unassign(category_id, product_id, store_id, idempotency_key)
category.unassign

Unassign category to product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | Defines category unassign, specified by category id | [required] |
**product_id** | **String** | Defines category unassign to the product, specified by product id | [required] |
**store_id** | Option<**String**> | Store Id |  |
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> |  |

### Return type

[**models::CategoryAssign200Response**](CategoryAssign_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## category_update

> models::AccountConfigUpdate200Response category_update(id, name, description, short_description, parent_id, avail, sort_order, modified_time, meta_title, meta_description, meta_keywords, seo_url, store_id, stores_ids, lang_id, idempotency_key)
category.update

Update category in store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Defines category update specified by category id | [required] |
**name** | Option<**String**> | Defines new category’s name |  |
**description** | Option<**String**> | Defines new category's description |  |
**short_description** | Option<**String**> | Defines short description |  |
**parent_id** | Option<**String**> | Defines new parent category id |  |
**avail** | Option<**bool**> | Defines category's visibility status |  |
**sort_order** | Option<**i32**> | Sort number in the list |  |
**modified_time** | Option<**String**> | Entity's date modification |  |
**meta_title** | Option<**String**> | Defines unique meta title for each entity |  |
**meta_description** | Option<**String**> | Defines unique meta description of a entity |  |
**meta_keywords** | Option<**String**> | Defines unique meta keywords for each entity |  |
**seo_url** | Option<**String**> | Defines unique category's URL for SEO |  |
**store_id** | Option<**String**> | Store Id |  |
**stores_ids** | Option<**String**> | Update category in the stores that is specified by comma-separated stores' id |  |
**lang_id** | Option<**String**> | Language id |  |
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> |  |

### Return type

[**models::AccountConfigUpdate200Response**](AccountConfigUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


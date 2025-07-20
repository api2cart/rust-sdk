# \CustomerApi

All URIs are relative to *https://api.api2cart.local.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_add**](CustomerApi.md#customer_add) | **POST** /customer.add.json | customer.add
[**customer_address_add**](CustomerApi.md#customer_address_add) | **POST** /customer.address.add.json | customer.address.add
[**customer_attribute_list**](CustomerApi.md#customer_attribute_list) | **GET** /customer.attribute.list.json | customer.attribute.list
[**customer_count**](CustomerApi.md#customer_count) | **GET** /customer.count.json | customer.count
[**customer_delete**](CustomerApi.md#customer_delete) | **DELETE** /customer.delete.json | customer.delete
[**customer_find**](CustomerApi.md#customer_find) | **GET** /customer.find.json | customer.find
[**customer_group_add**](CustomerApi.md#customer_group_add) | **POST** /customer.group.add.json | customer.group.add
[**customer_group_list**](CustomerApi.md#customer_group_list) | **GET** /customer.group.list.json | customer.group.list
[**customer_info**](CustomerApi.md#customer_info) | **GET** /customer.info.json | customer.info
[**customer_list**](CustomerApi.md#customer_list) | **GET** /customer.list.json | customer.list
[**customer_update**](CustomerApi.md#customer_update) | **PUT** /customer.update.json | customer.update
[**customer_wishlist_list**](CustomerApi.md#customer_wishlist_list) | **GET** /customer.wishlist.list.json | customer.wishlist.list



## customer_add

> models::CustomerAdd200Response customer_add(customer_add)
customer.add

Add customer into store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_add** | [**CustomerAdd**](CustomerAdd.md) |  | [required] |

### Return type

[**models::CustomerAdd200Response**](CustomerAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_address_add

> models::AttributeAdd200Response customer_address_add(customer_address_add)
customer.address.add

Add customer address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_address_add** | [**CustomerAddressAdd**](CustomerAddressAdd.md) |  | [required] |

### Return type

[**models::AttributeAdd200Response**](AttributeAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_attribute_list

> models::ModelResponseCustomerAttributeList customer_attribute_list(customer_id, count, page_cursor, store_id, lang_id, response_fields, params, exclude)
customer.attribute.list

Get attributes for specific customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Retrieves orders specified by customer id | [required] |
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to force_all]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseCustomerAttributeList**](Model_Response_Customer_Attribute_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_count

> models::CustomerCount200Response customer_count(ids, since_id, customer_list_id, group_id, store_id, avail, include_guests, find_value, find_where, created_from, created_to, modified_from, modified_to)
customer.count

Get number of customers from store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | Counts customers specified by ids |  |
**since_id** | Option<**String**> | Retrieve entities starting from the specified id. |  |
**customer_list_id** | Option<**String**> | The numeric ID of the customer list in Demandware. |  |
**group_id** | Option<**String**> | Customer group_id |  |
**store_id** | Option<**String**> | Counts customer specified by store id |  |
**avail** | Option<**bool**> | Defines category's visibility status |  |[default to true]
**include_guests** | Option<**bool**> | Indicates whether to include guest customers in the total count. |  |[default to false]
**find_value** | Option<**String**> | Entity search that is specified by some value |  |
**find_where** | Option<**String**> | Counts customers that are searched specified by field |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |

### Return type

[**models::CustomerCount200Response**](CustomerCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_delete

> models::CustomerDelete200Response customer_delete(id)
customer.delete

Delete customer from store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Identifies customer specified by the id | [required] |

### Return type

[**models::CustomerDelete200Response**](CustomerDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_find

> models::CustomerFind200Response customer_find(find_value, find_where, find_params, store_id, include_guests)
customer.find

Find customers in store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**find_value** | **String** | Entity search that is specified by some value | [required] |
**find_where** | Option<**String**> | Entity search that is specified by the comma-separated unique fields |  |[default to email]
**find_params** | Option<**String**> | Entity search that is specified by comma-separated parameters |  |[default to whole_words]
**store_id** | Option<**String**> | Store Id |  |
**include_guests** | Option<**bool**> | Indicates whether to search among guest customers when looking up a customer. |  |[default to false]

### Return type

[**models::CustomerFind200Response**](CustomerFind_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_group_add

> models::CustomerGroupAdd200Response customer_group_add(name, store_id, stores_ids)
customer.group.add

Create customer group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Customer group name | [required] |
**store_id** | Option<**String**> | Store Id |  |
**stores_ids** | Option<**String**> | Assign customer group to the stores that is specified by comma-separated stores' id |  |

### Return type

[**models::CustomerGroupAdd200Response**](CustomerGroupAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_group_list

> models::ModelResponseCustomerGroupList customer_group_list(start, count, page_cursor, group_ids, store_id, lang_id, response_fields, params, exclude, disable_cache)
customer.group.list

Get list of customers groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**group_ids** | Option<**String**> | Groups that will be assigned to a customer |  |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,name,additional_fields]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |
**disable_cache** | Option<**bool**> | Disable cache for current request |  |[default to false]

### Return type

[**models::ModelResponseCustomerGroupList**](Model_Response_Customer_Group_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_info

> models::CustomerInfo200Response customer_info(id, store_id, response_fields, params, exclude)
customer.info

Get customers' details from store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Retrieves customer's info specified by customer id | [required] |
**store_id** | Option<**String**> | Retrieves customer info specified by store id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,email,first_name,last_name]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::CustomerInfo200Response**](CustomerInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_list

> models::ModelResponseCustomerList customer_list(start, count, page_cursor, ids, since_id, customer_list_id, group_id, store_id, avail, include_guests, find_value, find_where, created_from, created_to, modified_from, modified_to, sort_by, sort_direction, response_fields, params, exclude)
customer.list

Get list of customers from store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**ids** | Option<**String**> | Retrieves customers specified by ids |  |
**since_id** | Option<**String**> | Retrieve entities starting from the specified id. |  |
**customer_list_id** | Option<**String**> | The numeric ID of the customer list in Demandware. |  |
**group_id** | Option<**String**> | Customer group_id |  |
**store_id** | Option<**String**> | Retrieves customers specified by store id |  |
**avail** | Option<**bool**> | Defines category's visibility status |  |[default to true]
**include_guests** | Option<**bool**> | Indicates whether to include guest customers in the list results. |  |[default to false]
**find_value** | Option<**String**> | Entity search that is specified by some value |  |
**find_where** | Option<**String**> | Customer search that is specified by field |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**sort_by** | Option<**String**> | Set field to sort by |  |[default to created_time]
**sort_direction** | Option<**String**> | Set sorting direction |  |[default to asc]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,email,first_name,last_name]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseCustomerList**](Model_Response_Customer_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_update

> models::AccountConfigUpdate200Response customer_update(customer_update)
customer.update

Update information of customer in store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_update** | [**CustomerUpdate**](CustomerUpdate.md) |  | [required] |

### Return type

[**models::AccountConfigUpdate200Response**](AccountConfigUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_wishlist_list

> models::ModelResponseCustomerWishlistList customer_wishlist_list(customer_id, start, count, page_cursor, id, store_id, response_fields)
customer.wishlist.list

Get a Wish List of customer from the store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Retrieves orders specified by customer id | [required] |
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**id** | Option<**String**> | Entity id |  |
**store_id** | Option<**String**> | Store Id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to {return_code,return_message,pagination,result}]

### Return type

[**models::ModelResponseCustomerWishlistList**](Model_Response_Customer_Wishlist_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AttributeApi

All URIs are relative to *https://api.api2cart.local.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attribute_add**](AttributeApi.md#attribute_add) | **POST** /attribute.add.json | attribute.add
[**attribute_assign_group**](AttributeApi.md#attribute_assign_group) | **POST** /attribute.assign.group.json | attribute.assign.group
[**attribute_assign_set**](AttributeApi.md#attribute_assign_set) | **POST** /attribute.assign.set.json | attribute.assign.set
[**attribute_attributeset_list**](AttributeApi.md#attribute_attributeset_list) | **GET** /attribute.attributeset.list.json | attribute.attributeset.list
[**attribute_count**](AttributeApi.md#attribute_count) | **GET** /attribute.count.json | attribute.count
[**attribute_delete**](AttributeApi.md#attribute_delete) | **DELETE** /attribute.delete.json | attribute.delete
[**attribute_group_list**](AttributeApi.md#attribute_group_list) | **GET** /attribute.group.list.json | attribute.group.list
[**attribute_info**](AttributeApi.md#attribute_info) | **GET** /attribute.info.json | attribute.info
[**attribute_list**](AttributeApi.md#attribute_list) | **GET** /attribute.list.json | attribute.list
[**attribute_type_list**](AttributeApi.md#attribute_type_list) | **GET** /attribute.type.list.json | attribute.type.list
[**attribute_unassign_group**](AttributeApi.md#attribute_unassign_group) | **POST** /attribute.unassign.group.json | attribute.unassign.group
[**attribute_unassign_set**](AttributeApi.md#attribute_unassign_set) | **POST** /attribute.unassign.set.json | attribute.unassign.set
[**attribute_update**](AttributeApi.md#attribute_update) | **PUT** /attribute.update.json | attribute.update
[**attribute_value_add**](AttributeApi.md#attribute_value_add) | **POST** /attribute.value.add.json | attribute.value.add
[**attribute_value_delete**](AttributeApi.md#attribute_value_delete) | **DELETE** /attribute.value.delete.json | attribute.value.delete
[**attribute_value_update**](AttributeApi.md#attribute_value_update) | **PUT** /attribute.value.update.json | attribute.value.update



## attribute_add

> models::AttributeAdd200Response attribute_add(r#type, name, code, store_id, lang_id, visible, required, position, attribute_group_id, is_global, is_searchable, is_filterable, is_comparable, is_html_allowed_on_front, is_filterable_in_search, is_configurable, is_visible_in_advanced_search, is_used_for_promo_rules, used_in_product_listing, used_for_sort_by, apply_to)
attribute.add

Add new attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Defines attribute's type | [required] |
**name** | **String** | Defines attributes's name | [required] |
**code** | Option<**String**> | Entity code |  |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |
**visible** | Option<**bool**> | Set visibility status |  |[default to false]
**required** | Option<**bool**> | Defines if the option is required |  |[default to false]
**position** | Option<**i32**> | Attribute`s position |  |[default to 0]
**attribute_group_id** | Option<**String**> | Filter by attribute_group_id |  |
**is_global** | Option<**String**> | Attribute saving scope |  |[default to Store]
**is_searchable** | Option<**bool**> | Use attribute in Quick Search |  |[default to false]
**is_filterable** | Option<**String**> | Use In Layered Navigation |  |[default to No]
**is_comparable** | Option<**bool**> | Comparable on Front-end |  |[default to false]
**is_html_allowed_on_front** | Option<**bool**> | Allow HTML Tags on Frontend |  |[default to false]
**is_filterable_in_search** | Option<**bool**> | Use In Search Results Layered Navigation |  |[default to false]
**is_configurable** | Option<**bool**> | Use To Create Configurable Product |  |[default to false]
**is_visible_in_advanced_search** | Option<**bool**> | Use in Advanced Search |  |[default to false]
**is_used_for_promo_rules** | Option<**bool**> | Use for Promo Rule Conditions |  |[default to false]
**used_in_product_listing** | Option<**bool**> | Used in Product Listing |  |[default to false]
**used_for_sort_by** | Option<**bool**> | Used for Sorting in Product Listing |  |[default to false]
**apply_to** | Option<**String**> | Types of products which can have this attribute |  |[default to all_types]

### Return type

[**models::AttributeAdd200Response**](AttributeAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_assign_group

> models::AttributeAssignGroup200Response attribute_assign_group(id, group_id, attribute_set_id)
attribute.assign.group

Assign attribute to the group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**group_id** | **String** | Attribute group_id | [required] |
**attribute_set_id** | Option<**String**> | Attribute set id |  |

### Return type

[**models::AttributeAssignGroup200Response**](AttributeAssignGroup_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_assign_set

> models::AttributeAssignGroup200Response attribute_assign_set(id, attribute_set_id, group_id)
attribute.assign.set

Assign attribute to the attribute set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**attribute_set_id** | **String** | Attribute set id | [required] |
**group_id** | Option<**String**> | Attribute group_id |  |

### Return type

[**models::AttributeAssignGroup200Response**](AttributeAssignGroup_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_attributeset_list

> models::ModelResponseAttributeAttributesetList attribute_attributeset_list(start, count, response_fields, params, exclude)
attribute.attributeset.list

Get attribute_set list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,name]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseAttributeAttributesetList**](Model_Response_Attribute_Attributeset_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_count

> models::AttributeCount200Response attribute_count(r#type, attribute_set_id, store_id, lang_id, visible, required, system)
attribute.count

Get attributes count

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Defines attribute's type |  |
**attribute_set_id** | Option<**String**> | Filter items by attribute set id |  |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |
**visible** | Option<**bool**> | Filter items by visibility status |  |
**required** | Option<**bool**> | Defines if the option is required |  |
**system** | Option<**bool**> | True if attribute is system |  |

### Return type

[**models::AttributeCount200Response**](AttributeCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_delete

> models::AttributeDelete200Response attribute_delete(id, store_id)
attribute.delete

Delete attribute from store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::AttributeDelete200Response**](AttributeDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_group_list

> models::ModelResponseAttributeGroupList attribute_group_list(start, count, attribute_set_id, lang_id, response_fields, params, exclude)
attribute.group.list

Get attribute group list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**attribute_set_id** | Option<**String**> | Attribute set id |  |
**lang_id** | Option<**String**> | Language id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,name]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseAttributeGroupList**](Model_Response_Attribute_Group_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_info

> models::AttributeInfo200Response attribute_info(id, attribute_set_id, store_id, lang_id, response_fields, params, exclude)
attribute.info

Get information about a specific global attribute by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**attribute_set_id** | Option<**String**> | Attribute set id |  |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to force_all]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::AttributeInfo200Response**](AttributeInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_list

> models::ModelResponseAttributeList attribute_list(start, count, attribute_ids, attribute_set_id, store_id, lang_id, r#type, visible, required, system, response_fields, params, exclude)
attribute.list

Get a list of global attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**attribute_ids** | Option<**String**> | Filter attributes by ids |  |
**attribute_set_id** | Option<**String**> | Filter items by attribute set id |  |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Retrieves attributes on specified language id |  |
**r#type** | Option<**String**> | Defines attribute's type |  |
**visible** | Option<**bool**> | Filter items by visibility status |  |
**required** | Option<**bool**> | Defines if the option is required |  |
**system** | Option<**bool**> | True if attribute is system |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,name,code,type]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseAttributeList**](Model_Response_Attribute_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_type_list

> models::AttributeTypeList200Response attribute_type_list()
attribute.type.list

Get list of supported attributes types

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AttributeTypeList200Response**](AttributeTypeList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_unassign_group

> models::AttributeUnassignGroup200Response attribute_unassign_group(id, group_id)
attribute.unassign.group

Unassign attribute from group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**group_id** | **String** | Customer group_id | [required] |

### Return type

[**models::AttributeUnassignGroup200Response**](AttributeUnassignGroup_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_unassign_set

> models::AttributeUnassignGroup200Response attribute_unassign_set(id, attribute_set_id)
attribute.unassign.set

Unassign attribute from attribute set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**attribute_set_id** | **String** | Attribute set id | [required] |

### Return type

[**models::AttributeUnassignGroup200Response**](AttributeUnassignGroup_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_update

> models::AttributeUpdate200Response attribute_update(id, name, store_id, lang_id)
attribute.update

Update attribute data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**name** | **String** | Defines new attributes's name | [required] |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |

### Return type

[**models::AttributeUpdate200Response**](AttributeUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_value_add

> models::AttributeAdd200Response attribute_value_add(attribute_id, name, code, description, store_id, lang_id)
attribute.value.add

Add new value to attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | Attribute Id | [required] |
**name** | **String** | Defines attribute value's name | [required] |
**code** | Option<**String**> | Entity code |  |
**description** | Option<**String**> | Defines attribute value's description |  |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |

### Return type

[**models::AttributeAdd200Response**](AttributeAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_value_delete

> models::AttributeValueDelete200Response attribute_value_delete(id, attribute_id, store_id)
attribute.value.delete

Delete attribute value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |
**attribute_id** | **String** | Attribute Id | [required] |
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::AttributeValueDelete200Response**](AttributeValueDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attribute_value_update

> models::AttributeUpdate200Response attribute_value_update(id, attribute_id, name, description, code, store_id, lang_id)
attribute.value.update

Update attribute value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Defines attribute value's id | [required] |
**attribute_id** | **String** | Attribute Id | [required] |
**name** | Option<**String**> | Defines attribute value's name |  |
**description** | Option<**String**> | Defines new attribute value's description |  |
**code** | Option<**String**> | Entity code |  |
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |

### Return type

[**models::AttributeUpdate200Response**](AttributeUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


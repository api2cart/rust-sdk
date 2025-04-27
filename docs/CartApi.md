# \CartApi

All URIs are relative to *https://api.api2cart.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cart_bridge**](CartApi.md#cart_bridge) | **GET** /cart.bridge.json | cart.bridge
[**cart_catalog_price_rules_count**](CartApi.md#cart_catalog_price_rules_count) | **GET** /cart.catalog_price_rules.count.json | cart.catalog_price_rules.count
[**cart_catalog_price_rules_list**](CartApi.md#cart_catalog_price_rules_list) | **GET** /cart.catalog_price_rules.list.json | cart.catalog_price_rules.list
[**cart_clear_cache**](CartApi.md#cart_clear_cache) | **POST** /cart.clear_cache.json | cart.clear_cache
[**cart_config**](CartApi.md#cart_config) | **GET** /cart.config.json | cart.config
[**cart_config_update**](CartApi.md#cart_config_update) | **PUT** /cart.config.update.json | cart.config.update
[**cart_coupon_add**](CartApi.md#cart_coupon_add) | **POST** /cart.coupon.add.json | cart.coupon.add
[**cart_coupon_condition_add**](CartApi.md#cart_coupon_condition_add) | **POST** /cart.coupon.condition.add.json | cart.coupon.condition.add
[**cart_coupon_count**](CartApi.md#cart_coupon_count) | **GET** /cart.coupon.count.json | cart.coupon.count
[**cart_coupon_delete**](CartApi.md#cart_coupon_delete) | **DELETE** /cart.coupon.delete.json | cart.coupon.delete
[**cart_coupon_list**](CartApi.md#cart_coupon_list) | **GET** /cart.coupon.list.json | cart.coupon.list
[**cart_create**](CartApi.md#cart_create) | **POST** /cart.create.json | cart.create
[**cart_delete**](CartApi.md#cart_delete) | **DELETE** /cart.delete.json | cart.delete
[**cart_disconnect**](CartApi.md#cart_disconnect) | **GET** /cart.disconnect.json | cart.disconnect
[**cart_giftcard_add**](CartApi.md#cart_giftcard_add) | **POST** /cart.giftcard.add.json | cart.giftcard.add
[**cart_giftcard_count**](CartApi.md#cart_giftcard_count) | **GET** /cart.giftcard.count.json | cart.giftcard.count
[**cart_giftcard_delete**](CartApi.md#cart_giftcard_delete) | **DELETE** /cart.giftcard.delete.json | cart.giftcard.delete
[**cart_giftcard_list**](CartApi.md#cart_giftcard_list) | **GET** /cart.giftcard.list.json | cart.giftcard.list
[**cart_info**](CartApi.md#cart_info) | **GET** /cart.info.json | cart.info
[**cart_list**](CartApi.md#cart_list) | **GET** /cart.list.json | cart.list
[**cart_meta_data_list**](CartApi.md#cart_meta_data_list) | **GET** /cart.meta_data.list.json | cart.meta_data.list
[**cart_meta_data_set**](CartApi.md#cart_meta_data_set) | **POST** /cart.meta_data.set.json | cart.meta_data.set
[**cart_meta_data_unset**](CartApi.md#cart_meta_data_unset) | **DELETE** /cart.meta_data.unset.json | cart.meta_data.unset
[**cart_methods**](CartApi.md#cart_methods) | **GET** /cart.methods.json | cart.methods
[**cart_plugin_list**](CartApi.md#cart_plugin_list) | **GET** /cart.plugin.list.json | cart.plugin.list
[**cart_script_add**](CartApi.md#cart_script_add) | **POST** /cart.script.add.json | cart.script.add
[**cart_script_delete**](CartApi.md#cart_script_delete) | **DELETE** /cart.script.delete.json | cart.script.delete
[**cart_script_list**](CartApi.md#cart_script_list) | **GET** /cart.script.list.json | cart.script.list
[**cart_shipping_zones_list**](CartApi.md#cart_shipping_zones_list) | **GET** /cart.shipping_zones.list.json | cart.shipping_zones.list
[**cart_validate**](CartApi.md#cart_validate) | **GET** /cart.validate.json | cart.validate



## cart_bridge

> models::CartBridge200Response cart_bridge()
cart.bridge

Get bridge key and store key

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CartBridge200Response**](CartBridge_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_catalog_price_rules_count

> models::CartCatalogPriceRulesCount200Response cart_catalog_price_rules_count()
cart.catalog_price_rules.count

Get count of cart catalog price rules discounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CartCatalogPriceRulesCount200Response**](CartCatalogPriceRulesCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_catalog_price_rules_list

> models::ModelResponseCartCatalogPriceRulesList cart_catalog_price_rules_list(start, count, page_cursor, ids, response_fields, params, exclude)
cart.catalog_price_rules.list

Get cart catalog price rules discounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**ids** | Option<**String**> | Retrieves  catalog_price_rules by ids |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,name,description]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseCartCatalogPriceRulesList**](Model_Response_Cart_Catalog_PriceRules_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_clear_cache

> models::CartClearCache200Response cart_clear_cache(cache_type)
cart.clear_cache

Clear cache on store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cache_type** | **String** | Defines which cache should be cleared. | [required] |

### Return type

[**models::CartClearCache200Response**](CartClearCache_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_config

> models::CartConfig200Response cart_config(params, exclude)
cart.config

Get list of cart configs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to store_name,store_url,db_prefix]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::CartConfig200Response**](CartConfig_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_config_update

> models::CartConfigUpdate200Response cart_config_update(cart_config_update)
cart.config.update

Use this API method to update custom data in client database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cart_config_update** | [**CartConfigUpdate**](CartConfigUpdate.md) |  | [required] |

### Return type

[**models::CartConfigUpdate200Response**](CartConfigUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_coupon_add

> models::CartCouponAdd200Response cart_coupon_add(cart_coupon_add)
cart.coupon.add

Use this method to create a coupon with specified conditions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cart_coupon_add** | [**CartCouponAdd**](CartCouponAdd.md) |  | [required] |

### Return type

[**models::CartCouponAdd200Response**](CartCouponAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_coupon_condition_add

> models::BasketLiveShippingServiceDelete200Response cart_coupon_condition_add(coupon_id, entity, key, operator, value, target, include_tax, include_shipping, store_id)
cart.coupon.condition.add

Use this method to add additional conditions for coupon application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coupon_id** | **String** | Coupon Id | [required] |
**entity** | **String** | Defines condition entity type | [required] |
**key** | **String** | Defines condition entity attribute key | [required] |
**operator** | **String** | Defines condition operator | [required] |
**value** | **String** | Defines condition value, can be comma separated according to the operator. | [required] |
**target** | Option<**String**> | Defines condition operator |  |[default to coupon_prerequisite]
**include_tax** | Option<**bool**> | Indicates whether to apply a discount for taxes. |  |[default to false]
**include_shipping** | Option<**bool**> | Indicates whether to apply a discount for shipping. |  |[default to false]
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::BasketLiveShippingServiceDelete200Response**](BasketLiveShippingServiceDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_coupon_count

> models::CartCouponCount200Response cart_coupon_count(store_id, avail, date_start_from, date_start_to, date_end_from, date_end_to)
cart.coupon.count

This method allows you to get the number of coupons. On some platforms, you can filter the coupons by the date they were active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**String**> | Store Id |  |
**avail** | Option<**bool**> | Defines category's visibility status |  |[default to true]
**date_start_from** | Option<**String**> | Filter entity by date_start (greater or equal) |  |
**date_start_to** | Option<**String**> | Filter entity by date_start (less or equal) |  |
**date_end_from** | Option<**String**> | Filter entity by date_end (greater or equal) |  |
**date_end_to** | Option<**String**> | Filter entity by date_end (less or equal) |  |

### Return type

[**models::CartCouponCount200Response**](CartCouponCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_coupon_delete

> models::AttributeDelete200Response cart_coupon_delete(id, store_id)
cart.coupon.delete

Delete coupon

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


## cart_coupon_list

> models::ModelResponseCartCouponList cart_coupon_list(start, count, page_cursor, coupons_ids, store_id, lang_id, avail, date_start_from, date_start_to, date_end_from, date_end_to, response_fields, params, exclude)
cart.coupon.list

Get cart coupon discounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**coupons_ids** | Option<**String**> | Filter coupons by ids |  |
**store_id** | Option<**String**> | Filter coupons by store id |  |
**lang_id** | Option<**String**> | Language id |  |
**avail** | Option<**bool**> | Filter coupons by avail status |  |
**date_start_from** | Option<**String**> | Filter entity by date_start (greater or equal) |  |
**date_start_to** | Option<**String**> | Filter entity by date_start (less or equal) |  |
**date_end_from** | Option<**String**> | Filter entity by date_end (greater or equal) |  |
**date_end_to** | Option<**String**> | Filter entity by date_end (less or equal) |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,code,name,description]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseCartCouponList**](Model_Response_Cart_Coupon_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_create

> models::AccountCartAdd200Response cart_create(cart_create)
cart.create

Add store to the account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cart_create** | [**CartCreate**](CartCreate.md) |  | [required] |

### Return type

[**models::AccountCartAdd200Response**](AccountCartAdd_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_delete

> models::CartDelete200Response cart_delete(delete_bridge)
cart.delete

Remove store from API2Cart

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_bridge** | Option<**bool**> | Identifies if there is a necessity to delete bridge |  |[default to true]

### Return type

[**models::CartDelete200Response**](CartDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_disconnect

> models::CartDisconnect200Response cart_disconnect(delete_bridge)
cart.disconnect

Disconnect with the store and clear store session data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_bridge** | Option<**bool**> | Identifies if there is a necessity to delete bridge |  |[default to false]

### Return type

[**models::CartDisconnect200Response**](CartDisconnect_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_giftcard_add

> models::CartGiftcardAdd200Response cart_giftcard_add(amount, code, owner_email, recipient_email, recipient_name, owner_name)
cart.giftcard.add

Use this method to create a gift card for a specified amount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amount** | **f64** | Defines the gift card amount value. | [required] |
**code** | Option<**String**> | Gift card code |  |
**owner_email** | Option<**String**> | Gift card owner email |  |
**recipient_email** | Option<**String**> | Gift card recipient email |  |
**recipient_name** | Option<**String**> | Gift card recipient name |  |
**owner_name** | Option<**String**> | Gift card owner name |  |

### Return type

[**models::CartGiftcardAdd200Response**](CartGiftcardAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_giftcard_count

> models::CartGiftcardCount200Response cart_giftcard_count(store_id)
cart.giftcard.count

Get gift cards count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::CartGiftcardCount200Response**](CartGiftcardCount_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_giftcard_delete

> models::AttributeDelete200Response cart_giftcard_delete(id)
cart.giftcard.delete

Delete giftcard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |

### Return type

[**models::AttributeDelete200Response**](AttributeDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_giftcard_list

> models::ModelResponseCartGiftCardList cart_giftcard_list(start, count, page_cursor, store_id, response_fields, params, exclude)
cart.giftcard.list

Get gift cards list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**store_id** | Option<**String**> | Store Id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,code,name]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseCartGiftCardList**](Model_Response_Cart_GiftCard_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_info

> models::CartInfo200Response cart_info(store_id, response_fields, params, exclude)
cart.info

This method allows you to get various information about the store, including a list of stores (in the case of a multistore configuration), a list of supported languages, currencies, carriers, warehouses, and many other information. This information contains data that is relatively stable and rarely changes, so API2Cart can cache certain data to reduce the load on the store and speed up the execution of the request. We also recommend that you cache the response of this method on your side to save requests. If you need to clear the cache for a specific store, then use the cart.validate method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**String**> | Store Id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to store_name,store_url,db_prefix]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::CartInfo200Response**](CartInfo_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_list

> models::CartList200Response cart_list()
cart.list

Get list of supported carts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CartList200Response**](CartList_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_meta_data_list

> models::ModelResponseCartMetaDataList cart_meta_data_list(entity_id, count, page_cursor, entity, store_id, lang_id, key, response_fields, params, exclude)
cart.meta_data.list

Using this method, you can get a list of metadata for various entities (products, options, customers, orders). Usually this is data created by third-party plugins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Entity Id | [required] |
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**entity** | Option<**String**> | Entity |  |[default to product]
**store_id** | Option<**String**> | Store Id |  |
**lang_id** | Option<**String**> | Language id |  |
**key** | Option<**String**> | Key |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to key,value]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseCartMetaDataList**](Model_Response_Cart_MetaData_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_meta_data_set

> models::AttributeAdd200Response cart_meta_data_set(entity_id, key, value, namespace, entity, store_id, lang_id)
cart.meta_data.set

Set meta data for a specific entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Entity Id | [required] |
**key** | **String** | Key | [required] |
**value** | **String** | Value | [required] |
**namespace** | **String** | Metafield namespace | [required] |
**entity** | Option<**String**> | Entity |  |[default to product]
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


## cart_meta_data_unset

> models::BasketLiveShippingServiceDelete200Response cart_meta_data_unset(entity_id, key, id, entity, store_id)
cart.meta_data.unset

Unset meta data for a specific entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Entity Id | [required] |
**key** | **String** | Key | [required] |
**id** | **String** | Entity id | [required] |
**entity** | Option<**String**> | Entity |  |[default to product]
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::BasketLiveShippingServiceDelete200Response**](BasketLiveShippingServiceDelete_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_methods

> models::CartMethods200Response cart_methods()
cart.methods

Returns a list of supported API methods.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CartMethods200Response**](CartMethods_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_plugin_list

> models::CartPluginList200Response cart_plugin_list(start, count, store_id)
cart.plugin.list

Get a list of third-party plugins installed on the store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::CartPluginList200Response**](CartPluginList_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_script_add

> models::CartScriptAdd200Response cart_script_add(name, description, html, src, load_method, scope, events, store_id)
cart.script.add

Add new script to the storefront

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The user-friendly script name |  |
**description** | Option<**String**> | The user-friendly description |  |
**html** | Option<**String**> | An html string containing exactly one `script` tag. |  |
**src** | Option<**String**> | The URL of the remote script |  |
**load_method** | Option<**String**> | The load method to use for the script |  |
**scope** | Option<**String**> | The page or pages on the online store where the script should be included |  |[default to storefront]
**events** | Option<**String**> | Event for run scripts |  |
**store_id** | Option<**String**> | Store Id |  |

### Return type

[**models::CartScriptAdd200Response**](CartScriptAdd_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_script_delete

> models::AttributeDelete200Response cart_script_delete(id, store_id)
cart.script.delete

Remove script from the storefront

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


## cart_script_list

> models::ModelResponseCartScriptList cart_script_list(start, count, page_cursor, script_ids, store_id, created_from, created_to, modified_from, modified_to, response_fields, params, exclude)
cart.script.list

Get scripts installed to the storefront

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**script_ids** | Option<**String**> | Retrieves only scripts with specific ids |  |
**store_id** | Option<**String**> | Store Id |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**modified_from** | Option<**String**> | Retrieve entities from their modification date |  |
**modified_to** | Option<**String**> | Retrieve entities to their modification date |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,name,description]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseCartScriptList**](Model_Response_Cart_Script_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_shipping_zones_list

> models::ModelResponseCartShippingZonesList cart_shipping_zones_list(start, count, store_id, response_fields, params, exclude)
cart.shipping_zones.list

Get list of shipping zones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**store_id** | Option<**String**> | Store Id |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to id,name,enabled]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseCartShippingZonesList**](Model_Response_Cart_ShippingZones_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cart_validate

> models::CartValidate200Response cart_validate(validate_version)
cart.validate

This method clears the cache in API2Cart for a particular store and checks whether the connection to the store is available. Use this method if there have been any changes in the settings on the storе, for example, if a new plugin has been installed or removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_version** | Option<**bool**> | Specify if api2cart should validate cart version |  |[default to false]

### Return type

[**models::CartValidate200Response**](CartValidate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


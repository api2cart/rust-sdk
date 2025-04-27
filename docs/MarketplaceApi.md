# \MarketplaceApi

All URIs are relative to *https://api.api2cart.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**marketplace_product_find**](MarketplaceApi.md#marketplace_product_find) | **GET** /marketplace.product.find.json | marketplace.product.find



## marketplace_product_find

> models::ModelResponseMarketplaceProductFind marketplace_product_find(count, page_cursor, keyword, categories_ids, store_id, asin, ean, gtin, upc, mpn, isbn, response_fields, params, exclude)
marketplace.product.find

Search product in global catalog.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**keyword** | Option<**String**> | Defines search keyword |  |
**categories_ids** | Option<**String**> | Defines product add that is specified by comma-separated categories id |  |
**store_id** | Option<**String**> | Store Id |  |
**asin** | Option<**String**> | Amazon Standard Identification Number. |  |
**ean** | Option<**String**> | European Article Number. An EAN is a unique 8 or 13-digit identifier that many industries (such as book publishers) use to identify products. |  |
**gtin** | Option<**String**> | Global Trade Item Number. An GTIN is an identifier for trade items. |  |
**upc** | Option<**String**> | Universal Product Code. A UPC (UPC-A) is a commonly used identifer for many different products. |  |
**mpn** | Option<**String**> | Manufacturer Part Number. A MPN is an identifier of a particular part design or material used. |  |
**isbn** | Option<**String**> | International Standard Book Number. An ISBN is a unique identifier for books. |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to force_all]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::ModelResponseMarketplaceProductFind**](Model_Response_Marketplace_Product_Find.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


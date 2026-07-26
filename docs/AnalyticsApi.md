# \AnalyticsApi

All URIs are relative to *https://api.api2cart.local.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_customer_report**](AnalyticsApi.md#analytics_customer_report) | **GET** /analytics.customer_report.json | analytics.customer_report
[**analytics_product_report**](AnalyticsApi.md#analytics_product_report) | **GET** /analytics.product_report.json | analytics.product_report
[**analytics_report**](AnalyticsApi.md#analytics_report) | **GET** /analytics.report.json | analytics.report



## analytics_customer_report

> models::ResponseAnalyticsCustomerReportResult analytics_customer_report(date_from, date_to, count, currency_id, store_id, customer_type, email, sort_by, sort_direction, page_cursor, response_fields)
analytics.customer_report

Get customer-level analytics for a store over a given period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | Option<**String**> | Start date for the analytics period (Y-m-d or Y-m-d H:i:s) |  |
**date_to** | Option<**String**> | End date for the analytics period (Y-m-d or Y-m-d H:i:s). Defaults to the current date. |  |
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**currency_id** | Option<**String**> | Currency Id |  |
**store_id** | Option<**String**> | Store Id |  |
**customer_type** | Option<**String**> | Filter analytics customers by customer type |  |
**email** | Option<**String**> | Filter analytics customers by email |  |
**sort_by** | Option<**String**> | Set field to sort by |  |[default to total_spend]
**sort_direction** | Option<**String**> | Set sorting direction |  |[default to desc]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**response_fields** | Option<**String**> | Set this parameter to choose which entity fields to retrieve. Use comma-separated field names in curly braces, nested to match the response structure, e.g. {result{product{id,name}}}. The wildcard * returns every field at a level: {*} gives the whole response, {result{product{*}}} all product fields. |  |

### Return type

[**models::ResponseAnalyticsCustomerReportResult**](Response_Analytics_CustomerReport_Result.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analytics_product_report

> models::ResponseAnalyticsProductReportResult analytics_product_report(date_from, date_to, count, product_ids, currency_id, store_id, categories_ids, sort_by, sort_direction, page_cursor, response_fields)
analytics.product_report

Get product-level analytics for a store over a given period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | Option<**String**> | Start date for the analytics period (Y-m-d or Y-m-d H:i:s) |  |
**date_to** | Option<**String**> | End date for the analytics period (Y-m-d or Y-m-d H:i:s). Defaults to the current date. |  |
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**product_ids** | Option<**String**> | Filter analytics by product ids |  |
**currency_id** | Option<**String**> | Currency Id |  |
**store_id** | Option<**String**> | Store Id |  |
**categories_ids** | Option<**String**> | Defines product add that is specified by comma-separated categories id |  |
**sort_by** | Option<**String**> | Set field to sort by |  |[default to items_sold]
**sort_direction** | Option<**String**> | Set sorting direction |  |[default to desc]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**response_fields** | Option<**String**> | Set this parameter to choose which entity fields to retrieve. Use comma-separated field names in curly braces, nested to match the response structure, e.g. {result{product{id,name}}}. The wildcard * returns every field at a level: {*} gives the whole response, {result{product{*}}} all product fields. |  |

### Return type

[**models::ResponseAnalyticsProductReportResult**](Response_Analytics_ProductReport_Result.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analytics_report

> models::ResponseAnalyticsReportResult analytics_report(date_from, date_to, interval, order_status, financial_status, currency_id, store_id, sort_by, sort_direction, response_fields)
analytics.report

Get analytics report with totals and optional interval breakdown for a store over a given period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_from** | **String** | Start date for the analytics period (Y-m-d or Y-m-d H:i:s) | [required] |
**date_to** | Option<**String**> | End date for the analytics period (Y-m-d or Y-m-d H:i:s). Defaults to the current date. |  |
**interval** | Option<**String**> | Interval for analytics report breakdown |  |
**order_status** | Option<**String**> | Retrieves orders specified by order status |  |
**financial_status** | Option<**String**> | Retrieves orders specified by financial status |  |
**currency_id** | Option<**String**> | Currency Id |  |
**store_id** | Option<**String**> | Store Id |  |
**sort_by** | Option<**String**> | Set field to sort by |  |[default to date]
**sort_direction** | Option<**String**> | Set sorting direction |  |[default to asc]
**response_fields** | Option<**String**> | Set this parameter to choose which entity fields to retrieve. Use comma-separated field names in curly braces, nested to match the response structure, e.g. {result{product{id,name}}}. The wildcard * returns every field at a level: {*} gives the whole response, {result{product{*}}} all product fields. |  |

### Return type

[**models::ResponseAnalyticsReportResult**](Response_Analytics_Report_Result.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


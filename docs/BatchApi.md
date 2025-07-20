# \BatchApi

All URIs are relative to *https://api.api2cart.local.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_job_list**](BatchApi.md#batch_job_list) | **GET** /batch.job.list.json | batch.job.list
[**batch_job_result**](BatchApi.md#batch_job_result) | **GET** /batch.job.result.json | batch.job.result



## batch_job_list

> models::ModelResponseBatchJobList batch_job_list(count, page_cursor, ids, created_from, created_to, processed_from, processed_to, response_fields)
batch.job.list

Get list of recent jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**page_cursor** | Option<**String**> | Used to retrieve entities via cursor-based pagination (it can't be used with any other filtering parameter) |  |
**ids** | Option<**String**> | Filter batch jobs by ids |  |
**created_from** | Option<**String**> | Retrieve entities from their creation date |  |
**created_to** | Option<**String**> | Retrieve entities to their creation date |  |
**processed_from** | Option<**String**> | Retrieve entities according to their processing datetime |  |
**processed_to** | Option<**String**> | Retrieve entities according to their processing datetime |  |
**response_fields** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to {return_code,return_message,pagination,result}]

### Return type

[**models::ModelResponseBatchJobList**](Model_Response_Batch_Job_List.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_job_result

> models::ResponseBatchJobResult batch_job_result(id)
batch.job.result

Get job result data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Entity id | [required] |

### Return type

[**models::ResponseBatchJobResult**](Response_Batch_Job_Result.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# ProductImageAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Defines image's types that are specified by comma-separated list | 
**image_name** | **String** | Defines image's name | 
**product_id** | Option<**String**> | Defines product id where the image should be added | [optional]
**product_variant_id** | Option<**String**> | Defines product's variants specified by variant id | [optional]
**variant_ids** | Option<**String**> | Defines product's variants ids | [optional]
**option_value_ids** | Option<**String**> | Defines product's option values ids | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**lang_id** | Option<**String**> | Add product image on specified language id | [optional]
**url** | Option<**String**> | Defines URL of the image that has to be added | [optional]
**content** | Option<**String**> | Content(body) encoded in base64 of image file | [optional]
**label** | Option<**String**> | Defines alternative text that has to be attached to the picture | [optional]
**mime** | Option<**String**> | Mime type of image http://en.wikipedia.org/wiki/Internet_media_type. | [optional]
**position** | Option<**i32**> | Defines image’s position in the list | [optional][default to 0]
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version | [optional][default to false]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



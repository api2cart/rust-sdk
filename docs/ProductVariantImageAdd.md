# ProductVariantImageAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | Option<**String**> | Defines product id where the variant image has to be added | [optional]
**product_variant_id** | **String** | Defines product's variants specified by variant id | 
**store_id** | Option<**String**> | Store Id | [optional]
**image_name** | **String** | Defines image's name | 
**r#type** | **String** | Defines image's types that are specified by comma-separated list | [default to Base]
**url** | Option<**String**> | Defines URL of the image that has to be added | [optional]
**content** | Option<**String**> | Content(body) encoded in base64 of image file | [optional]
**label** | Option<**String**> | Defines alternative text that has to be attached to the picture | [optional]
**mime** | Option<**String**> | Mime type of image http://en.wikipedia.org/wiki/Internet_media_type. | [optional]
**position** | Option<**i32**> | Defines image’s position in the list | [optional][default to 0]
**option_id** | Option<**String**> | Defines option id of the product variant for which the image will be added | [optional]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



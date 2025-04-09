# TaxClassRate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**tax** | Option<**f64**> |  | [optional]
**tax_type** | Option<**i32**> |  | [optional]
**tax_based_on** | Option<**String**> |  | [optional]
**countries** | Option<[**Vec<models::TaxClassCountries>**](TaxClass_Countries.md)> |  | [optional]
**cities** | Option<**Vec<String>**> |  | [optional]
**address** | Option<**Vec<String>**> |  | [optional]
**zip_codes** | Option<[**Vec<models::TaxClassZipCodes>**](TaxClass_ZipCodes.md)> |  | [optional]
**additional_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# Customer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**phone** | Option<**String**> |  | [optional]
**created_time** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**modified_time** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**group** | Option<[**Vec<models::CustomerGroup>**](Customer_Group.md)> |  | [optional]
**login** | Option<**String**> |  | [optional]
**last_login** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**birth_day** | Option<[**models::A2CDateTime**](A2CDateTime.md)> |  | [optional]
**status** | Option<**String**> |  | [optional]
**news_letter_subscription** | Option<**bool**> |  | [optional]
**consents** | Option<[**Vec<models::CustomerConsent>**](Customer_Consent.md)> |  | [optional]
**gender** | Option<**String**> |  | [optional]
**stores_ids** | Option<**Vec<String>**> |  | [optional]
**website** | Option<**String**> |  | [optional]
**fax** | Option<**String**> |  | [optional]
**company** | Option<**String**> |  | [optional]
**ip_address** | Option<**String**> |  | [optional]
**address_book** | Option<[**Vec<models::CustomerAddress>**](Customer_Address.md)> |  | [optional]
**lang_id** | Option<**String**> |  | [optional]
**orders_count** | Option<**i32**> |  | [optional]
**last_order_id** | Option<**String**> |  | [optional]
**additional_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



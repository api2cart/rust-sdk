# CustomerUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Entity id | [optional]
**group_id** | Option<**String**> | Customer group_id | [optional]
**group_ids** | Option<**String**> | Groups that will be assigned to a customer | [optional]
**group** | Option<**String**> | Defines the group where the customer | [optional]
**email** | Option<**String**> | Defines customer's email | [optional]
**phone** | Option<**String**> | Defines customer's phone number | [optional]
**first_name** | Option<**String**> | Defines customer's first name | [optional]
**last_name** | Option<**String**> | Defines customer's last name | [optional]
**birth_day** | Option<**String**> | Defines customer's birthday | [optional]
**news_letter_subscription** | Option<**bool**> | Defines whether the newsletter subscription is available for the user | [optional]
**consents** | Option<[**Vec<models::CustomerAddConsentsInner>**](CustomerAdd_consents_inner.md)> | Defines consents to notifications | [optional]
**tags** | Option<**String**> | Customer tags | [optional]
**gender** | Option<**String**> | Defines customer's gender | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**note** | Option<**String**> | The customer note. | [optional]
**status** | Option<**String**> | Defines customer's status | [optional]
**address** | Option<[**Vec<models::CustomerUpdateAddressInner>**](CustomerUpdate_address_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



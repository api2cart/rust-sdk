# CustomerAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Defines customer's email | 
**first_name** | **String** | Defines customer's first name | 
**last_name** | **String** | Defines customer's last name | 
**password** | Option<**String**> | Defines customer's unique password | [optional]
**group** | Option<**String**> | Defines the group where the customer | [optional]
**group_ids** | Option<**String**> | Groups that will be assigned to a customer | [optional]
**created_time** | Option<**String**> | Entity's date creation | [optional]
**modified_time** | Option<**String**> | Entity's date modification | [optional]
**login** | Option<**String**> | Specifies customer's login name | [optional]
**last_login** | Option<**String**> | Defines customer's last login time | [optional]
**birth_day** | Option<**String**> | Defines customer's birthday | [optional]
**status** | Option<**String**> | Defines customer's status | [optional][default to enabled]
**news_letter_subscription** | Option<**bool**> | Defines whether the newsletter subscription is available for the user | [optional][default to false]
**consents** | Option<[**Vec<models::CustomerAddConsentsInner>**](CustomerAdd_consents_inner.md)> | Defines consents to notifications | [optional]
**gender** | Option<**String**> | Defines customer's gender | [optional]
**website** | Option<**String**> | Link to customer website | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**fax** | Option<**String**> | Defines customer's fax | [optional]
**company** | Option<**String**> | Defines customer's company | [optional]
**phone** | Option<**String**> | Defines customer's phone number | [optional]
**note** | Option<**String**> | The customer note. | [optional]
**country** | Option<**String**> | Specifies ISO code or name of country | [optional]
**address** | Option<[**Vec<models::CustomerAddAddressInner>**](CustomerAdd_address_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



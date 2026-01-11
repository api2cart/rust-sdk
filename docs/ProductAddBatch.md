# ProductAddBatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nested_items_update_behaviour** | Option<**String**> |  Determines how updates to nested items should be handled.<hr><div style=\"font-style:normal\">  Values description:  <div style=\"margin-left: 2%; padding-top: 2%\">    <div style=\"font-size:85%\">      <b>  replace</b>: This option indicates that the nested items should be completely replaced with the new data provided. </br>      <b>  merge</b>: With this option, updates to nested items are merged with the existing data. </br>    </div>  </div></div> | [optional][default to Replace]
**clear_cache** | Option<**bool**> |  | [optional][default to false]
**reindex** | Option<**bool**> |  | [optional][default to false]
**payload** | [**Vec<models::ProductAddBatchPayloadInner>**](ProductAddBatch_payload_inner.md) | Contains an array of product objects. The list of properties may vary depending on the specific platform. | 
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# OrderShipmentUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | Shipment id indicates the number of delivery | 
**order_id** | Option<**String**> | Defines the order that will be updated | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**shipment_provider** | Option<**String**> | Defines company name that provide tracking of shipment | [optional]
**tracking_numbers** | Option<[**Vec<models::OrderShipmentAddTrackingNumbersInner>**](OrderShipmentAdd_tracking_numbers_inner.md)> | Defines shipment's tracking numbers that have to be added</br> How set tracking numbers to appropriate carrier:<ul><li>tracking_numbers[]=a2c.demo1,a2c.demo2 - set default carrier</li><li>tracking_numbers[<b>carrier_id</b>]=a2c.demo - set appropriate carrier</li></ul>To get the list of carriers IDs that are available in your store, use the <a href = \"https://api2cart.com/docs/#/cart/CartInfo\">cart.info</a > method | [optional]
**tracking_link** | Option<**String**> | Defines custom tracking link | [optional]
**is_shipped** | Option<**bool**> | Defines shipment's status | [optional][default to true]
**delivered_at** | Option<**String**> | Defines the date of delivery | [optional]
**replace** | Option<**bool**> | Allows rewrite tracking numbers | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



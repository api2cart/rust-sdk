# OrderShipmentAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | Option<**String**> | Defines the order for which the shipment will be created | [optional]
**warehouse_id** | Option<**String**> | This parameter is used for selecting a warehouse where you need to set/modify a product quantity. | [optional]
**store_id** | Option<**String**> | Store Id | [optional]
**shipment_provider** | Option<**String**> | Defines company name that provide tracking of shipment | [optional]
**shipping_method** | Option<**String**> | Define shipping method | [optional]
**items** | Option<[**Vec<models::OrderShipmentAddItemsInner>**](OrderShipmentAdd_items_inner.md)> | Defines items in the order that will be shipped | [optional]
**tracking_numbers** | Option<[**Vec<models::OrderShipmentAddTrackingNumbersInner>**](OrderShipmentAdd_tracking_numbers_inner.md)> | Defines shipment's tracking numbers that have to be added</br> How set tracking numbers to appropriate carrier:<ul><li>tracking_numbers[]=a2c.demo1,a2c.demo2 - set default carrier</li><li>tracking_numbers[<b>carrier_id</b>]=a2c.demo - set appropriate carrier</li></ul>To get the list of carriers IDs that are available in your store, use the <a href = \"https://api2cart.com/docs/#/cart/CartInfo\">cart.info</a > method | [optional]
**tracking_link** | Option<**String**> | Defines custom tracking link | [optional]
**is_shipped** | Option<**bool**> | Defines shipment's status | [optional][default to true]
**send_notifications** | Option<**bool**> | Send notifications to customer after shipment was created | [optional][default to false]
**adjust_stock** | Option<**bool**> | This parameter is used for adjust stock. | [optional][default to false]
**enable_cache** | Option<**bool**> | If the value is 'true' and order exist in our cache, we will use order.info from cache to prepare shipment items. | [optional][default to false]
**check_process_status** | Option<**bool**> | Disable or enable check process status. Please note that the response will be slower due to additional requests to the store. | [optional][default to false]
**tracking_provider** | Option<**String**> | Defines name of the company which provides shipment tracking | [optional]
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



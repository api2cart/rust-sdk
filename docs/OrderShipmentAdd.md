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
**check_process_status** | Option<**bool**> | Disable or enable check process status. Please note that the response will be slower due to additional requests to the store. | [optional][default to false]
**tracking_provider** | Option<**String**> | Defines name of the company which provides shipment tracking | [optional]
**admin_comment** | Option<**String**> | Specifies admin's order comment | [optional]
**mail_class** | Option<**String**> | Mail class for the shipment (e.g., priority, express). | [optional]
**ship_date** | Option<**String**> | Ship date. | [optional]
**weight** | Option<**f64**> | Weight | [optional]
**weight_unit** | Option<**String**> | Weight Unit | [optional]
**length** | Option<**f64**> | Defines product's length | [optional]
**width** | Option<**f64**> | Defines product's width | [optional]
**height** | Option<**f64**> | Defines product's height | [optional]
**dimensions_unit** | Option<**String**> | Weight Unit | [optional]
**shipping_label_cost** | Option<**f64**> | Cost of the shipping label. | [optional]
**shipping_label_currency** | Option<**String**> | Currency code for the shipping label cost (3-letter ISO code). | [optional]
**revenue_eligibility** | Option<**bool**> | Revenue eligibility flag. | [optional]
**ship_from_country** | Option<**String**> | Country code the shipment is sent from (2-letter ISO code). | [optional]
**ship_to_country** | Option<**String**> | Country code the shipment is sent to (2-letter ISO code). | [optional]
**incoterm** | Option<**String**> | International commercial term for the shipment (e.g., DAP, DDP). | [optional]
**duty_amount** | Option<**f64**> | Duty amount for international shipment. | [optional]
**duty_currency** | Option<**String**> | Currency code for the duty amount (3-letter ISO code). | [optional]
**enable_cache** | Option<**bool**> | If the value is 'true' and order exist in our cache, we will use order.info from cache to prepare shipment items. | [optional][default to false]
**use_latest_api_version** | Option<**bool**> | Use the latest platform API version | [optional][default to false]
**idempotency_key** | Option<**String**> | A unique identifier associated with a specific request. Repeated requests with the same <strong>idempotency_key</strong> return a cached response without re-executing the business logic. <strong>Please note that the cache lifetime is 15 minutes.</strong> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



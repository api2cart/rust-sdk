# \AccountApi

All URIs are relative to *https://api.api2cart.com/v1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_cart_add**](AccountApi.md#account_cart_add) | **POST** /account.cart.add.json | account.cart.add
[**account_cart_list**](AccountApi.md#account_cart_list) | **GET** /account.cart.list.json | account.cart.list
[**account_config_update**](AccountApi.md#account_config_update) | **PUT** /account.config.update.json | account.config.update
[**account_failed_webhooks**](AccountApi.md#account_failed_webhooks) | **GET** /account.failed_webhooks.json | account.failed_webhooks
[**account_supported_platforms**](AccountApi.md#account_supported_platforms) | **GET** /account.supported_platforms.json | account.supported_platforms



## account_cart_add

> models::AccountCartAdd200Response account_cart_add(account_cart_add)
account.cart.add

Use this method to automate the process of connecting stores to API2Cart. The list of parameters will vary depending on the platform. To get a list of parameters that are specific to a particular shopping platform, you need to execute the account.supported_platforms.json method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_cart_add** | [**AccountCartAdd**](AccountCartAdd.md) |  | [required] |

### Return type

[**models::AccountCartAdd200Response**](AccountCartAdd_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_cart_list

> models::AccountCartList200Response account_cart_list(store_url, store_key, request_from_date, request_to_date, params, exclude)
account.cart.list

This method lets you get a list of online stores connected to your API2Cart account. You can get the number of API requests to each store if you specify a period using parameters (request_from_date, request_to_date). The total_calls field is displayed only if there are parameters (request_from_date, request_to_date).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_url** | Option<**String**> | A web address of a store |  |
**store_key** | Option<**String**> | Find store by store key |  |
**request_from_date** | Option<**String**> | Retrieve entities from their creation date |  |
**request_to_date** | Option<**String**> | Retrieve entities to their creation date |  |
**params** | Option<**String**> | Set this parameter in order to choose which entity fields you want to retrieve |  |[default to force_all]
**exclude** | Option<**String**> | Set this parameter in order to choose which entity fields you want to ignore. Works only if parameter `params` equal force_all |  |

### Return type

[**models::AccountCartList200Response**](AccountCartList_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_config_update

> models::AccountConfigUpdate200Response account_config_update(replace_parameters, new_store_url, new_store_key, bridge_url, store_root, db_tables_prefix, user_agent, param_3dcart_private_key, param_3dcart_access_token, param_3dcartapi_api_key, amazon_sp_client_id, amazon_sp_client_secret, amazon_sp_refresh_token, amazon_sp_aws_region, amazon_sp_api_environment, amazon_seller_id, aspdotnetstorefront_api_user, aspdotnetstorefront_api_pass, bigcommerceapi_admin_account, bigcommerceapi_api_path, bigcommerceapi_api_key, bigcommerceapi_client_id, bigcommerceapi_access_token, bigcommerceapi_context, bol_api_key, bol_api_secret, bol_retailer_id, demandware_client_id, demandware_api_password, demandware_user_name, demandware_user_password, ebay_client_id, ebay_client_secret, ebay_runame, ebay_access_token, ebay_refresh_token, ebay_environment, ebay_site_id, ecwid_acess_token, ecwid_store_id, lazada_app_id, lazada_app_secret, lazada_refresh_token, lazada_region, etsy_keystring, etsy_shared_secret, etsy_access_token, etsy_token_secret, etsy_client_id, etsy_refresh_token, facebook_app_id, facebook_app_secret, facebook_access_token, facebook_business_id, neto_api_key, neto_api_username, shopline_access_token, shopline_app_key, shopline_app_secret, shopline_shared_secret, shopify_access_token, shopify_api_key, shopify_api_password, shopify_shared_secret, shoplazza_access_token, shoplazza_shared_secret, miva_access_token, miva_signature, shopware_access_key, shopware_api_key, shopware_api_secret, bigcartel_user_name, bigcartel_password, volusion_login, volusion_password, walmart_client_id, walmart_client_secret, walmart_environment, walmart_channel_type, walmart_region, square_client_id, square_client_secret, square_refresh_token, squarespace_api_key, squarespace_client_id, squarespace_client_secret, squarespace_access_token, squarespace_refresh_token, hybris_client_id, hybris_client_secret, hybris_username, hybris_password, hybris_websites, lightspeed_api_key, lightspeed_api_secret, commercehq_api_key, commercehq_api_password, wc_consumer_key, wc_consumer_secret, magento_consumer_key, magento_consumer_secret, magento_access_token, magento_token_secret, prestashop_webservice_key, wix_app_id, wix_app_secret_key, wix_instance_id, wix_refresh_token, mercado_libre_app_id, mercado_libre_app_secret_key, mercado_libre_refresh_token, zid_client_id, zid_client_secret, zid_access_token, zid_authorization, zid_refresh_token, flipkart_client_id, flipkart_client_secret, allegro_client_id, allegro_client_secret, allegro_access_token, allegro_refresh_token, allegro_environment, zoho_client_id, zoho_client_secret, zoho_refresh_token, zoho_region, tiendanube_user_id, tiendanube_access_token, tiendanube_client_secret, otto_client_id, otto_client_secret, otto_app_id, otto_refresh_token, otto_environment, otto_access_token, tiktokshop_app_key, tiktokshop_app_secret, tiktokshop_refresh_token, tiktokshop_access_token, salla_client_id, salla_client_secret, salla_refresh_token, salla_access_token)
account.config.update

Use this method to automate the change of credentials used to connect online stores. The list of supported parameters differs depending on the platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**replace_parameters** | Option<**bool**> | Identifies if there is a necessity to replace parameters |  |
**new_store_url** | Option<**String**> | The web address of the store you want to update to connect to API2Cart |  |
**new_store_key** | Option<**String**> | Update store key |  |
**bridge_url** | Option<**String**> | This parameter allows to set up store with custom bridge url (also you must use store_root parameter if a bridge folder is not in the root folder of the store) |  |
**store_root** | Option<**String**> | Absolute path to the store root directory (used with \"bridge_url\" parameter) |  |
**db_tables_prefix** | Option<**String**> | DB tables prefix |  |
**user_agent** | Option<**String**> | This parameter allows you to set your custom user agent, which will be used in requests to the store. Please use it cautiously, as the store's firewall may block specific values. |  |
**param_3dcart_private_key** | Option<**String**> | 3DCart Private Key |  |
**param_3dcart_access_token** | Option<**String**> | 3DCart Token |  |
**param_3dcartapi_api_key** | Option<**String**> | 3DCart API Key |  |
**amazon_sp_client_id** | Option<**String**> | Amazon SP API app client id |  |
**amazon_sp_client_secret** | Option<**String**> | Amazon SP API app client secret |  |
**amazon_sp_refresh_token** | Option<**String**> | Amazon SP API OAuth refresh token |  |
**amazon_sp_aws_region** | Option<**String**> | Amazon AWS Region |  |
**amazon_sp_api_environment** | Option<**String**> | Amazon SP API environment |  |[default to production]
**amazon_seller_id** | Option<**String**> | Amazon Seller ID (Merchant token) |  |
**aspdotnetstorefront_api_user** | Option<**String**> | It's a AspDotNetStorefront account for which API is available |  |
**aspdotnetstorefront_api_pass** | Option<**String**> | AspDotNetStorefront API Password |  |
**bigcommerceapi_admin_account** | Option<**String**> | It's a BigCommerce account for which API is enabled |  |
**bigcommerceapi_api_path** | Option<**String**> | BigCommerce API URL |  |
**bigcommerceapi_api_key** | Option<**String**> | Bigcommerce API Key |  |
**bigcommerceapi_client_id** | Option<**String**> | Client ID of the requesting app |  |
**bigcommerceapi_access_token** | Option<**String**> | Access token authorizing the app to access resources on behalf of a user |  |
**bigcommerceapi_context** | Option<**String**> | API Path section unique to the store |  |
**bol_api_key** | Option<**String**> | Bol API Key |  |
**bol_api_secret** | Option<**String**> | Bol API Secret |  |
**bol_retailer_id** | Option<**i32**> | Bol Retailer ID |  |
**demandware_client_id** | Option<**String**> | Demandware client id |  |
**demandware_api_password** | Option<**String**> | Demandware api password |  |
**demandware_user_name** | Option<**String**> | Demandware user name |  |
**demandware_user_password** | Option<**String**> | Demandware user password |  |
**ebay_client_id** | Option<**String**> | Application ID (AppID). |  |
**ebay_client_secret** | Option<**String**> | Shared Secret from eBay application |  |
**ebay_runame** | Option<**String**> | The RuName value that eBay assigns to your application. |  |
**ebay_access_token** | Option<**String**> | Used to authenticate API requests. |  |
**ebay_refresh_token** | Option<**String**> | Used to renew the access token. |  |
**ebay_environment** | Option<**String**> | eBay environment |  |
**ebay_site_id** | Option<**i32**> | eBay global ID |  |[default to 0]
**ecwid_acess_token** | Option<**String**> | Access token authorizing the app to access resources on behalf of a user |  |
**ecwid_store_id** | Option<**String**> | Store Id |  |
**lazada_app_id** | Option<**String**> | Lazada App ID |  |
**lazada_app_secret** | Option<**String**> | Lazada App Secret |  |
**lazada_refresh_token** | Option<**String**> | Lazada Refresh Token |  |
**lazada_region** | Option<**String**> | Lazada API endpoint Region |  |
**etsy_keystring** | Option<**String**> | Etsy keystring |  |
**etsy_shared_secret** | Option<**String**> | Etsy shared secret |  |
**etsy_access_token** | Option<**String**> | Access token authorizing the app to access resources on behalf of a user |  |
**etsy_token_secret** | Option<**String**> | Secret token authorizing the app to access resources on behalf of a user |  |
**etsy_client_id** | Option<**String**> | Etsy Client Id |  |
**etsy_refresh_token** | Option<**String**> | Etsy Refresh token |  |
**facebook_app_id** | Option<**String**> | Facebook App ID |  |
**facebook_app_secret** | Option<**String**> | Facebook App Secret |  |
**facebook_access_token** | Option<**String**> | Facebook Access Token |  |
**facebook_business_id** | Option<**String**> | Facebook Business ID |  |
**neto_api_key** | Option<**String**> | Neto API Key |  |
**neto_api_username** | Option<**String**> | Neto User Name |  |
**shopline_access_token** | Option<**String**> | Shopline APP Key |  |
**shopline_app_key** | Option<**String**> | Shopline APP Key |  |
**shopline_app_secret** | Option<**String**> | Shopline App Secret |  |
**shopline_shared_secret** | Option<**String**> | Shopline Shared Secret |  |
**shopify_access_token** | Option<**String**> | Access token authorizing the app to access resources on behalf of a user |  |
**shopify_api_key** | Option<**String**> | Shopify API Key |  |
**shopify_api_password** | Option<**String**> | Shopify API Password |  |
**shopify_shared_secret** | Option<**String**> | Shared secret |  |
**shoplazza_access_token** | Option<**String**> | Access token authorizing the app to access resources on behalf of a user |  |
**shoplazza_shared_secret** | Option<**String**> | Shared secret |  |
**miva_access_token** | Option<**String**> | Miva access token |  |
**miva_signature** | Option<**String**> | Miva signature |  |
**shopware_access_key** | Option<**String**> | Shopware access key |  |
**shopware_api_key** | Option<**String**> | Shopware api key |  |
**shopware_api_secret** | Option<**String**> | Shopware client secret access key |  |
**bigcartel_user_name** | Option<**String**> | Subdomain of store |  |
**bigcartel_password** | Option<**String**> | BigCartel account password |  |
**volusion_login** | Option<**String**> | It's a Volusion account for which API is enabled |  |
**volusion_password** | Option<**String**> | Volusion API Password |  |
**walmart_client_id** | Option<**String**> | Walmart client ID. For the region 'ca' use Consumer ID |  |
**walmart_client_secret** | Option<**String**> | Walmart client secret. For the region 'ca' use Private Key |  |
**walmart_environment** | Option<**String**> | Walmart environment |  |[default to production]
**walmart_channel_type** | Option<**String**> | Walmart WM_CONSUMER.CHANNEL.TYPE header |  |
**walmart_region** | Option<**String**> | Walmart region |  |[default to us]
**square_client_id** | Option<**String**> | Square (Weebly) Client ID |  |
**square_client_secret** | Option<**String**> | Square (Weebly) Client Secret |  |
**square_refresh_token** | Option<**String**> | Square (Weebly) Refresh Token |  |
**squarespace_api_key** | Option<**String**> | Squarespace API Key |  |
**squarespace_client_id** | Option<**String**> | Squarespace Connector Client ID |  |
**squarespace_client_secret** | Option<**String**> | Squarespace Connector Client Secret |  |
**squarespace_access_token** | Option<**String**> | Squarespace access token |  |
**squarespace_refresh_token** | Option<**String**> | Squarespace refresh token |  |
**hybris_client_id** | Option<**String**> | Omni Commerce Connector Client ID |  |
**hybris_client_secret** | Option<**String**> | Omni Commerce Connector Client Secret |  |
**hybris_username** | Option<**String**> | User Name |  |
**hybris_password** | Option<**String**> | User password |  |
**hybris_websites** | Option<[**Vec<String>**](String.md)> | Websites to stores mapping data |  |
**lightspeed_api_key** | Option<**String**> | LightSpeed api key |  |
**lightspeed_api_secret** | Option<**String**> | LightSpeed api secret |  |
**commercehq_api_key** | Option<**String**> | CommerceHQ api key |  |
**commercehq_api_password** | Option<**String**> | CommerceHQ api password |  |
**wc_consumer_key** | Option<**String**> | Woocommerce consumer key |  |
**wc_consumer_secret** | Option<**String**> | Woocommerce consumer secret |  |
**magento_consumer_key** | Option<**String**> | Magento Consumer Key |  |
**magento_consumer_secret** | Option<**String**> | Magento Consumer Secret |  |
**magento_access_token** | Option<**String**> | Magento Access Token |  |
**magento_token_secret** | Option<**String**> | Magento Token Secret |  |
**prestashop_webservice_key** | Option<**String**> | Prestashop webservice key |  |
**wix_app_id** | Option<**String**> | Wix App ID |  |
**wix_app_secret_key** | Option<**String**> | Wix App Secret Key |  |
**wix_instance_id** | Option<**String**> | Wix Instance ID |  |
**wix_refresh_token** | Option<**String**> | Wix refresh token |  |
**mercado_libre_app_id** | Option<**String**> | Mercado Libre App ID |  |
**mercado_libre_app_secret_key** | Option<**String**> | Mercado Libre App Secret Key |  |
**mercado_libre_refresh_token** | Option<**String**> | Mercado Libre Refresh Token |  |
**zid_client_id** | Option<**i32**> | Zid Client ID |  |
**zid_client_secret** | Option<**String**> | Zid Client Secret |  |
**zid_access_token** | Option<**String**> | Zid Access Token |  |
**zid_authorization** | Option<**String**> | Zid Authorization |  |
**zid_refresh_token** | Option<**String**> | Zid refresh token |  |
**flipkart_client_id** | Option<**String**> | Flipkart Client ID |  |
**flipkart_client_secret** | Option<**String**> | Flipkart Client Secret |  |
**allegro_client_id** | Option<**String**> | Allegro Client ID |  |
**allegro_client_secret** | Option<**String**> | Allegro Client Secret |  |
**allegro_access_token** | Option<**String**> | Allegro Access Token |  |
**allegro_refresh_token** | Option<**String**> | Allegro Refresh Token |  |
**allegro_environment** | Option<**String**> | Allegro Environment |  |[default to production]
**zoho_client_id** | Option<**String**> | Zoho Client ID |  |
**zoho_client_secret** | Option<**String**> | Zoho Client Secret |  |
**zoho_refresh_token** | Option<**String**> | Zoho Refresh Token |  |
**zoho_region** | Option<**String**> | Zoho API endpoint Region |  |
**tiendanube_user_id** | Option<**i32**> | Tiendanube User ID |  |
**tiendanube_access_token** | Option<**String**> | Tiendanube Access Token |  |
**tiendanube_client_secret** | Option<**String**> | Tiendanube Client Secret |  |
**otto_client_id** | Option<**String**> | Otto Client ID |  |
**otto_client_secret** | Option<**String**> | Otto Client Secret |  |
**otto_app_id** | Option<**String**> | Otto App ID |  |
**otto_refresh_token** | Option<**String**> | Otto Refresh Token |  |
**otto_environment** | Option<**String**> | Otto Environment |  |
**otto_access_token** | Option<**String**> | Otto Access Token |  |
**tiktokshop_app_key** | Option<**String**> | TikTok Shop App Key |  |
**tiktokshop_app_secret** | Option<**String**> | TikTok Shop App Secret |  |
**tiktokshop_refresh_token** | Option<**String**> | TikTok Shop Refresh Token |  |
**tiktokshop_access_token** | Option<**String**> | TikTok Shop Access Token |  |
**salla_client_id** | Option<**String**> | Salla Client ID |  |
**salla_client_secret** | Option<**String**> | Salla Client Secret |  |
**salla_refresh_token** | Option<**String**> | Salla Refresh Token |  |
**salla_access_token** | Option<**String**> | Salla Access Token |  |

### Return type

[**models::AccountConfigUpdate200Response**](AccountConfigUpdate_200_response.md)

### Authorization

[StoreKeyAuth](../README.md#StoreKeyAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_failed_webhooks

> models::AccountFailedWebhooks200Response account_failed_webhooks(start, count, ids)
account.failed_webhooks

If the callback of your service for some reason could not accept webhooks from API2Cart, then with the help of this method you can get a list of missed webhooks to perform synchronization again using entity_id. Please note that we keep such records for 24 hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | This parameter sets the number from which you want to get entities |  |[default to 0]
**count** | Option<**i32**> | This parameter sets the entity amount that has to be retrieved. Max allowed count=250 |  |[default to 10]
**ids** | Option<**String**> | List of сomma-separated webhook ids |  |

### Return type

[**models::AccountFailedWebhooks200Response**](AccountFailedWebhooks_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_supported_platforms

> models::AccountSupportedPlatforms200Response account_supported_platforms()
account.supported_platforms

Use this method to retrieve a list of supported platforms and the sets of parameters required for connecting to each of them. Note: some platforms may have multiple connection methods so that the response will contain multiple sets of parameters.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountSupportedPlatforms200Response**](AccountSupportedPlatforms_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


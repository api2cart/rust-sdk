/*
 * API2Cart OpenAPI
 *
 * API2Cart
 *
 * The version of the OpenAPI document: 1.1
 * Contact: contact@api2cart.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountCartAdd {
    /// Store’s identifier which you can get from cart_list method
    #[serde(rename = "cart_id")]
    pub cart_id: CartId,
    /// A web address of a store that you would like to connect to API2Cart
    #[serde(rename = "store_url", skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
    /// This parameter allows to set up store with custom bridge url (also you must use store_root parameter if a bridge folder is not in the root folder of the store)
    #[serde(rename = "bridge_url", skip_serializing_if = "Option::is_none")]
    pub bridge_url: Option<String>,
    /// Absolute path to the store root directory (used with \"bridge_url\" parameter)
    #[serde(rename = "store_root", skip_serializing_if = "Option::is_none")]
    pub store_root: Option<String>,
    /// Set this parameter if bridge is already uploaded to store
    #[serde(rename = "store_key", skip_serializing_if = "Option::is_none")]
    pub store_key: Option<String>,
    /// Specify if api2cart should validate cart version
    #[serde(rename = "validate_version", skip_serializing_if = "Option::is_none")]
    pub validate_version: Option<bool>,
    /// Enables or disables cart's verification
    #[serde(rename = "verify", skip_serializing_if = "Option::is_none")]
    pub verify: Option<bool>,
    /// DB tables prefix
    #[serde(rename = "db_tables_prefix", skip_serializing_if = "Option::is_none")]
    pub db_tables_prefix: Option<String>,
    /// This parameter allows you to set your custom user agent, which will be used in requests to the store. Please use it cautiously, as the store's firewall may block specific values.
    #[serde(rename = "user_agent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// FTP connection host
    #[serde(rename = "ftp_host", skip_serializing_if = "Option::is_none")]
    pub ftp_host: Option<String>,
    /// FTP User
    #[serde(rename = "ftp_user", skip_serializing_if = "Option::is_none")]
    pub ftp_user: Option<String>,
    /// FTP Password
    #[serde(rename = "ftp_password", skip_serializing_if = "Option::is_none")]
    pub ftp_password: Option<String>,
    /// FTP Port
    #[serde(rename = "ftp_port", skip_serializing_if = "Option::is_none")]
    pub ftp_port: Option<i32>,
    /// FTP Store dir
    #[serde(rename = "ftp_store_dir", skip_serializing_if = "Option::is_none")]
    pub ftp_store_dir: Option<String>,
    /// 3DCart Private Key
    #[serde(rename = "3dcart_private_key", skip_serializing_if = "Option::is_none")]
    pub param_3dcart_private_key: Option<String>,
    /// 3DCart Token
    #[serde(rename = "3dcart_access_token", skip_serializing_if = "Option::is_none")]
    pub param_3dcart_access_token: Option<String>,
    /// 3DCart API Key
    #[serde(rename = "3dcartapi_api_key", skip_serializing_if = "Option::is_none")]
    pub param_3dcartapi_api_key: Option<String>,
    /// Amazon SP API app client id
    #[serde(rename = "amazon_sp_client_id", skip_serializing_if = "Option::is_none")]
    pub amazon_sp_client_id: Option<String>,
    /// Amazon SP API app client secret
    #[serde(rename = "amazon_sp_client_secret", skip_serializing_if = "Option::is_none")]
    pub amazon_sp_client_secret: Option<String>,
    /// Amazon SP API OAuth refresh token
    #[serde(rename = "amazon_sp_refresh_token", skip_serializing_if = "Option::is_none")]
    pub amazon_sp_refresh_token: Option<String>,
    /// Amazon AWS Region
    #[serde(rename = "amazon_sp_aws_region", skip_serializing_if = "Option::is_none")]
    pub amazon_sp_aws_region: Option<String>,
    /// Amazon SP API environment
    #[serde(rename = "amazon_sp_api_environment", skip_serializing_if = "Option::is_none")]
    pub amazon_sp_api_environment: Option<String>,
    /// Amazon Seller ID (Merchant token)
    #[serde(rename = "amazon_seller_id", skip_serializing_if = "Option::is_none")]
    pub amazon_seller_id: Option<String>,
    /// It's a AspDotNetStorefront account for which API is available
    #[serde(rename = "aspdotnetstorefront_api_user", skip_serializing_if = "Option::is_none")]
    pub aspdotnetstorefront_api_user: Option<String>,
    /// AspDotNetStorefront API Password
    #[serde(rename = "aspdotnetstorefront_api_pass", skip_serializing_if = "Option::is_none")]
    pub aspdotnetstorefront_api_pass: Option<String>,
    /// It's a BigCommerce account for which API is enabled
    #[serde(rename = "bigcommerceapi_admin_account", skip_serializing_if = "Option::is_none")]
    pub bigcommerceapi_admin_account: Option<String>,
    /// BigCommerce API URL
    #[serde(rename = "bigcommerceapi_api_path", skip_serializing_if = "Option::is_none")]
    pub bigcommerceapi_api_path: Option<String>,
    /// Bigcommerce API Key
    #[serde(rename = "bigcommerceapi_api_key", skip_serializing_if = "Option::is_none")]
    pub bigcommerceapi_api_key: Option<String>,
    /// Client ID of the requesting app
    #[serde(rename = "bigcommerceapi_client_id", skip_serializing_if = "Option::is_none")]
    pub bigcommerceapi_client_id: Option<String>,
    /// Access token authorizing the app to access resources on behalf of a user
    #[serde(rename = "bigcommerceapi_access_token", skip_serializing_if = "Option::is_none")]
    pub bigcommerceapi_access_token: Option<String>,
    /// API Path section unique to the store
    #[serde(rename = "bigcommerceapi_context", skip_serializing_if = "Option::is_none")]
    pub bigcommerceapi_context: Option<String>,
    /// Bol API Key
    #[serde(rename = "bol_api_key", skip_serializing_if = "Option::is_none")]
    pub bol_api_key: Option<String>,
    /// Bol API Secret
    #[serde(rename = "bol_api_secret", skip_serializing_if = "Option::is_none")]
    pub bol_api_secret: Option<String>,
    /// Bol Retailer ID
    #[serde(rename = "bol_retailer_id", skip_serializing_if = "Option::is_none")]
    pub bol_retailer_id: Option<i32>,
    /// Demandware client id
    #[serde(rename = "demandware_client_id", skip_serializing_if = "Option::is_none")]
    pub demandware_client_id: Option<String>,
    /// Demandware api password
    #[serde(rename = "demandware_api_password", skip_serializing_if = "Option::is_none")]
    pub demandware_api_password: Option<String>,
    /// Demandware user name
    #[serde(rename = "demandware_user_name", skip_serializing_if = "Option::is_none")]
    pub demandware_user_name: Option<String>,
    /// Demandware user password
    #[serde(rename = "demandware_user_password", skip_serializing_if = "Option::is_none")]
    pub demandware_user_password: Option<String>,
    /// Application ID (AppID).
    #[serde(rename = "ebay_client_id", skip_serializing_if = "Option::is_none")]
    pub ebay_client_id: Option<String>,
    /// Shared Secret from eBay application
    #[serde(rename = "ebay_client_secret", skip_serializing_if = "Option::is_none")]
    pub ebay_client_secret: Option<String>,
    /// The RuName value that eBay assigns to your application.
    #[serde(rename = "ebay_runame", skip_serializing_if = "Option::is_none")]
    pub ebay_runame: Option<String>,
    /// Used to authenticate API requests.
    #[serde(rename = "ebay_access_token", skip_serializing_if = "Option::is_none")]
    pub ebay_access_token: Option<String>,
    /// Used to renew the access token.
    #[serde(rename = "ebay_refresh_token", skip_serializing_if = "Option::is_none")]
    pub ebay_refresh_token: Option<String>,
    /// eBay environment
    #[serde(rename = "ebay_environment", skip_serializing_if = "Option::is_none")]
    pub ebay_environment: Option<String>,
    /// eBay global ID
    #[serde(rename = "ebay_site_id", skip_serializing_if = "Option::is_none")]
    pub ebay_site_id: Option<i32>,
    /// Walmart client ID. For the region 'ca' use Consumer ID
    #[serde(rename = "walmart_client_id", skip_serializing_if = "Option::is_none")]
    pub walmart_client_id: Option<String>,
    /// Walmart client secret. For the region 'ca' use Private Key
    #[serde(rename = "walmart_client_secret", skip_serializing_if = "Option::is_none")]
    pub walmart_client_secret: Option<String>,
    /// Walmart environment
    #[serde(rename = "walmart_environment", skip_serializing_if = "Option::is_none")]
    pub walmart_environment: Option<String>,
    /// Walmart WM_CONSUMER.CHANNEL.TYPE header
    #[serde(rename = "walmart_channel_type", skip_serializing_if = "Option::is_none")]
    pub walmart_channel_type: Option<String>,
    /// Walmart region
    #[serde(rename = "walmart_region", skip_serializing_if = "Option::is_none")]
    pub walmart_region: Option<String>,
    /// Access token authorizing the app to access resources on behalf of a user
    #[serde(rename = "ecwid_acess_token", skip_serializing_if = "Option::is_none")]
    pub ecwid_acess_token: Option<String>,
    /// Store Id
    #[serde(rename = "ecwid_store_id", skip_serializing_if = "Option::is_none")]
    pub ecwid_store_id: Option<String>,
    /// Lazada App ID
    #[serde(rename = "lazada_app_id", skip_serializing_if = "Option::is_none")]
    pub lazada_app_id: Option<String>,
    /// Lazada App Secret
    #[serde(rename = "lazada_app_secret", skip_serializing_if = "Option::is_none")]
    pub lazada_app_secret: Option<String>,
    /// Lazada Refresh Token
    #[serde(rename = "lazada_refresh_token", skip_serializing_if = "Option::is_none")]
    pub lazada_refresh_token: Option<String>,
    /// Lazada API endpoint Region
    #[serde(rename = "lazada_region", skip_serializing_if = "Option::is_none")]
    pub lazada_region: Option<String>,
    /// LightSpeed api key
    #[serde(rename = "lightspeed_api_key", skip_serializing_if = "Option::is_none")]
    pub lightspeed_api_key: Option<String>,
    /// LightSpeed api secret
    #[serde(rename = "lightspeed_api_secret", skip_serializing_if = "Option::is_none")]
    pub lightspeed_api_secret: Option<String>,
    /// Etsy keystring
    #[serde(rename = "etsy_keystring", skip_serializing_if = "Option::is_none")]
    pub etsy_keystring: Option<String>,
    /// Etsy shared secret
    #[serde(rename = "etsy_shared_secret", skip_serializing_if = "Option::is_none")]
    pub etsy_shared_secret: Option<String>,
    /// Access token authorizing the app to access resources on behalf of a user
    #[serde(rename = "etsy_access_token", skip_serializing_if = "Option::is_none")]
    pub etsy_access_token: Option<String>,
    /// Secret token authorizing the app to access resources on behalf of a user
    #[serde(rename = "etsy_token_secret", skip_serializing_if = "Option::is_none")]
    pub etsy_token_secret: Option<String>,
    /// Etsy Client Id
    #[serde(rename = "etsy_client_id", skip_serializing_if = "Option::is_none")]
    pub etsy_client_id: Option<String>,
    /// Etsy Refresh token
    #[serde(rename = "etsy_refresh_token", skip_serializing_if = "Option::is_none")]
    pub etsy_refresh_token: Option<String>,
    /// Facebook App ID
    #[serde(rename = "facebook_app_id", skip_serializing_if = "Option::is_none")]
    pub facebook_app_id: Option<String>,
    /// Facebook App Secret
    #[serde(rename = "facebook_app_secret", skip_serializing_if = "Option::is_none")]
    pub facebook_app_secret: Option<String>,
    /// Facebook Access Token
    #[serde(rename = "facebook_access_token", skip_serializing_if = "Option::is_none")]
    pub facebook_access_token: Option<String>,
    /// Facebook Business ID
    #[serde(rename = "facebook_business_id", skip_serializing_if = "Option::is_none")]
    pub facebook_business_id: Option<String>,
    /// Neto API Key
    #[serde(rename = "neto_api_key", skip_serializing_if = "Option::is_none")]
    pub neto_api_key: Option<String>,
    /// Neto User Name
    #[serde(rename = "neto_api_username", skip_serializing_if = "Option::is_none")]
    pub neto_api_username: Option<String>,
    /// Shopline APP Key
    #[serde(rename = "shopline_access_token", skip_serializing_if = "Option::is_none")]
    pub shopline_access_token: Option<String>,
    /// Shopline APP Key
    #[serde(rename = "shopline_app_key", skip_serializing_if = "Option::is_none")]
    pub shopline_app_key: Option<String>,
    /// Shopline App Secret
    #[serde(rename = "shopline_app_secret", skip_serializing_if = "Option::is_none")]
    pub shopline_app_secret: Option<String>,
    /// Access token authorizing the app to access resources on behalf of a user
    #[serde(rename = "shopify_access_token", skip_serializing_if = "Option::is_none")]
    pub shopify_access_token: Option<String>,
    /// Shopify API Key
    #[serde(rename = "shopify_api_key", skip_serializing_if = "Option::is_none")]
    pub shopify_api_key: Option<String>,
    /// Shopify API Password
    #[serde(rename = "shopify_api_password", skip_serializing_if = "Option::is_none")]
    pub shopify_api_password: Option<String>,
    /// Shared secret
    #[serde(rename = "shopify_shared_secret", skip_serializing_if = "Option::is_none")]
    pub shopify_shared_secret: Option<String>,
    /// Access token authorizing the app to access resources on behalf of a user
    #[serde(rename = "shoplazza_access_token", skip_serializing_if = "Option::is_none")]
    pub shoplazza_access_token: Option<String>,
    /// Shared secret
    #[serde(rename = "shoplazza_shared_secret", skip_serializing_if = "Option::is_none")]
    pub shoplazza_shared_secret: Option<String>,
    /// Shopware access key
    #[serde(rename = "shopware_access_key", skip_serializing_if = "Option::is_none")]
    pub shopware_access_key: Option<String>,
    /// Shopware api key
    #[serde(rename = "shopware_api_key", skip_serializing_if = "Option::is_none")]
    pub shopware_api_key: Option<String>,
    /// Shopware client secret access key
    #[serde(rename = "shopware_api_secret", skip_serializing_if = "Option::is_none")]
    pub shopware_api_secret: Option<String>,
    /// Miva access token
    #[serde(rename = "miva_access_token", skip_serializing_if = "Option::is_none")]
    pub miva_access_token: Option<String>,
    /// Miva signature
    #[serde(rename = "miva_signature", skip_serializing_if = "Option::is_none")]
    pub miva_signature: Option<String>,
    /// Tiendanube User ID
    #[serde(rename = "tiendanube_user_id", skip_serializing_if = "Option::is_none")]
    pub tiendanube_user_id: Option<i32>,
    /// Tiendanube Access Token
    #[serde(rename = "tiendanube_access_token", skip_serializing_if = "Option::is_none")]
    pub tiendanube_access_token: Option<String>,
    /// Tiendanube Client Secret
    #[serde(rename = "tiendanube_client_secret", skip_serializing_if = "Option::is_none")]
    pub tiendanube_client_secret: Option<String>,
    /// It's a Volusion account for which API is enabled
    #[serde(rename = "volusion_login", skip_serializing_if = "Option::is_none")]
    pub volusion_login: Option<String>,
    /// Volusion API Password
    #[serde(rename = "volusion_password", skip_serializing_if = "Option::is_none")]
    pub volusion_password: Option<String>,
    /// Omni Commerce Connector Client ID
    #[serde(rename = "hybris_client_id", skip_serializing_if = "Option::is_none")]
    pub hybris_client_id: Option<String>,
    /// Omni Commerce Connector Client Secret
    #[serde(rename = "hybris_client_secret", skip_serializing_if = "Option::is_none")]
    pub hybris_client_secret: Option<String>,
    /// User Name
    #[serde(rename = "hybris_username", skip_serializing_if = "Option::is_none")]
    pub hybris_username: Option<String>,
    /// User password
    #[serde(rename = "hybris_password", skip_serializing_if = "Option::is_none")]
    pub hybris_password: Option<String>,
    /// Websites to stores mapping data
    #[serde(rename = "hybris_websites", skip_serializing_if = "Option::is_none")]
    pub hybris_websites: Option<Vec<models::AccountCartAddHybrisWebsitesInner>>,
    /// Square (Weebly) Client ID
    #[serde(rename = "square_client_id", skip_serializing_if = "Option::is_none")]
    pub square_client_id: Option<String>,
    /// Square (Weebly) Client Secret
    #[serde(rename = "square_client_secret", skip_serializing_if = "Option::is_none")]
    pub square_client_secret: Option<String>,
    /// Square (Weebly) Refresh Token
    #[serde(rename = "square_refresh_token", skip_serializing_if = "Option::is_none")]
    pub square_refresh_token: Option<String>,
    /// Squarespace API Key
    #[serde(rename = "squarespace_api_key", skip_serializing_if = "Option::is_none")]
    pub squarespace_api_key: Option<String>,
    /// Squarespace Connector Client ID
    #[serde(rename = "squarespace_client_id", skip_serializing_if = "Option::is_none")]
    pub squarespace_client_id: Option<String>,
    /// Squarespace Connector Client Secret
    #[serde(rename = "squarespace_client_secret", skip_serializing_if = "Option::is_none")]
    pub squarespace_client_secret: Option<String>,
    /// Squarespace access token
    #[serde(rename = "squarespace_access_token", skip_serializing_if = "Option::is_none")]
    pub squarespace_access_token: Option<String>,
    /// Squarespace refresh token
    #[serde(rename = "squarespace_refresh_token", skip_serializing_if = "Option::is_none")]
    pub squarespace_refresh_token: Option<String>,
    /// CommerceHQ api key
    #[serde(rename = "commercehq_api_key", skip_serializing_if = "Option::is_none")]
    pub commercehq_api_key: Option<String>,
    /// CommerceHQ api password
    #[serde(rename = "commercehq_api_password", skip_serializing_if = "Option::is_none")]
    pub commercehq_api_password: Option<String>,
    /// Woocommerce consumer key
    #[serde(rename = "wc_consumer_key", skip_serializing_if = "Option::is_none")]
    pub wc_consumer_key: Option<String>,
    /// Woocommerce consumer secret
    #[serde(rename = "wc_consumer_secret", skip_serializing_if = "Option::is_none")]
    pub wc_consumer_secret: Option<String>,
    /// Magento Consumer Key
    #[serde(rename = "magento_consumer_key", skip_serializing_if = "Option::is_none")]
    pub magento_consumer_key: Option<String>,
    /// Magento Consumer Secret
    #[serde(rename = "magento_consumer_secret", skip_serializing_if = "Option::is_none")]
    pub magento_consumer_secret: Option<String>,
    /// Magento Access Token
    #[serde(rename = "magento_access_token", skip_serializing_if = "Option::is_none")]
    pub magento_access_token: Option<String>,
    /// Magento Token Secret
    #[serde(rename = "magento_token_secret", skip_serializing_if = "Option::is_none")]
    pub magento_token_secret: Option<String>,
    /// Prestashop webservice key
    #[serde(rename = "prestashop_webservice_key", skip_serializing_if = "Option::is_none")]
    pub prestashop_webservice_key: Option<String>,
    /// Wix App ID
    #[serde(rename = "wix_app_id")]
    pub wix_app_id: String,
    /// Wix App Secret Key
    #[serde(rename = "wix_app_secret_key")]
    pub wix_app_secret_key: String,
    /// Wix Instance ID
    #[serde(rename = "wix_instance_id", skip_serializing_if = "Option::is_none")]
    pub wix_instance_id: Option<String>,
    /// Wix refresh token
    #[serde(rename = "wix_refresh_token", skip_serializing_if = "Option::is_none")]
    pub wix_refresh_token: Option<String>,
    /// Mercado Libre App ID
    #[serde(rename = "mercado_libre_app_id", skip_serializing_if = "Option::is_none")]
    pub mercado_libre_app_id: Option<String>,
    /// Mercado Libre App Secret Key
    #[serde(rename = "mercado_libre_app_secret_key", skip_serializing_if = "Option::is_none")]
    pub mercado_libre_app_secret_key: Option<String>,
    /// Mercado Libre Refresh Token
    #[serde(rename = "mercado_libre_refresh_token", skip_serializing_if = "Option::is_none")]
    pub mercado_libre_refresh_token: Option<String>,
    /// Zid Client ID
    #[serde(rename = "zid_client_id", skip_serializing_if = "Option::is_none")]
    pub zid_client_id: Option<i32>,
    /// Zid Client Secret
    #[serde(rename = "zid_client_secret", skip_serializing_if = "Option::is_none")]
    pub zid_client_secret: Option<String>,
    /// Zid Access Token
    #[serde(rename = "zid_access_token", skip_serializing_if = "Option::is_none")]
    pub zid_access_token: Option<String>,
    /// Zid Authorization
    #[serde(rename = "zid_authorization", skip_serializing_if = "Option::is_none")]
    pub zid_authorization: Option<String>,
    /// Zid refresh token
    #[serde(rename = "zid_refresh_token", skip_serializing_if = "Option::is_none")]
    pub zid_refresh_token: Option<String>,
    /// Flipkart Client ID
    #[serde(rename = "flipkart_client_id", skip_serializing_if = "Option::is_none")]
    pub flipkart_client_id: Option<String>,
    /// Flipkart Client Secret
    #[serde(rename = "flipkart_client_secret", skip_serializing_if = "Option::is_none")]
    pub flipkart_client_secret: Option<String>,
    /// Allegro Client ID
    #[serde(rename = "allegro_client_id", skip_serializing_if = "Option::is_none")]
    pub allegro_client_id: Option<String>,
    /// Allegro Client Secret
    #[serde(rename = "allegro_client_secret", skip_serializing_if = "Option::is_none")]
    pub allegro_client_secret: Option<String>,
    /// Allegro Access Token
    #[serde(rename = "allegro_access_token", skip_serializing_if = "Option::is_none")]
    pub allegro_access_token: Option<String>,
    /// Allegro Refresh Token
    #[serde(rename = "allegro_refresh_token", skip_serializing_if = "Option::is_none")]
    pub allegro_refresh_token: Option<String>,
    /// Allegro Environment
    #[serde(rename = "allegro_environment", skip_serializing_if = "Option::is_none")]
    pub allegro_environment: Option<String>,
    /// Zoho Client ID
    #[serde(rename = "zoho_client_id", skip_serializing_if = "Option::is_none")]
    pub zoho_client_id: Option<String>,
    /// Zoho Client Secret
    #[serde(rename = "zoho_client_secret", skip_serializing_if = "Option::is_none")]
    pub zoho_client_secret: Option<String>,
    /// Zoho Refresh Token
    #[serde(rename = "zoho_refresh_token", skip_serializing_if = "Option::is_none")]
    pub zoho_refresh_token: Option<String>,
    /// Zoho API endpoint Region
    #[serde(rename = "zoho_region", skip_serializing_if = "Option::is_none")]
    pub zoho_region: Option<String>,
    /// Otto Client ID
    #[serde(rename = "otto_client_id", skip_serializing_if = "Option::is_none")]
    pub otto_client_id: Option<String>,
    /// Otto Client Secret
    #[serde(rename = "otto_client_secret", skip_serializing_if = "Option::is_none")]
    pub otto_client_secret: Option<String>,
    /// Otto App ID
    #[serde(rename = "otto_app_id", skip_serializing_if = "Option::is_none")]
    pub otto_app_id: Option<String>,
    /// Otto Refresh Token
    #[serde(rename = "otto_refresh_token", skip_serializing_if = "Option::is_none")]
    pub otto_refresh_token: Option<String>,
    /// Otto Environment
    #[serde(rename = "otto_environment", skip_serializing_if = "Option::is_none")]
    pub otto_environment: Option<String>,
    /// Otto Access Token
    #[serde(rename = "otto_access_token", skip_serializing_if = "Option::is_none")]
    pub otto_access_token: Option<String>,
    /// TikTok Shop App Key
    #[serde(rename = "tiktokshop_app_key", skip_serializing_if = "Option::is_none")]
    pub tiktokshop_app_key: Option<String>,
    /// TikTok Shop App Secret
    #[serde(rename = "tiktokshop_app_secret", skip_serializing_if = "Option::is_none")]
    pub tiktokshop_app_secret: Option<String>,
    /// TikTok Shop Refresh Token
    #[serde(rename = "tiktokshop_refresh_token", skip_serializing_if = "Option::is_none")]
    pub tiktokshop_refresh_token: Option<String>,
    /// TikTok Shop Access Token
    #[serde(rename = "tiktokshop_access_token", skip_serializing_if = "Option::is_none")]
    pub tiktokshop_access_token: Option<String>,
    /// Salla Client ID
    #[serde(rename = "salla_client_id", skip_serializing_if = "Option::is_none")]
    pub salla_client_id: Option<String>,
    /// Salla Client Secret
    #[serde(rename = "salla_client_secret", skip_serializing_if = "Option::is_none")]
    pub salla_client_secret: Option<String>,
    /// Salla Refresh Token
    #[serde(rename = "salla_refresh_token", skip_serializing_if = "Option::is_none")]
    pub salla_refresh_token: Option<String>,
    /// Salla Access Token
    #[serde(rename = "salla_access_token", skip_serializing_if = "Option::is_none")]
    pub salla_access_token: Option<String>,
}

impl AccountCartAdd {
    pub fn new(cart_id: CartId, wix_app_id: String, wix_app_secret_key: String) -> AccountCartAdd {
        AccountCartAdd {
            cart_id,
            store_url: None,
            bridge_url: None,
            store_root: None,
            store_key: None,
            validate_version: None,
            verify: None,
            db_tables_prefix: None,
            user_agent: None,
            ftp_host: None,
            ftp_user: None,
            ftp_password: None,
            ftp_port: None,
            ftp_store_dir: None,
            param_3dcart_private_key: None,
            param_3dcart_access_token: None,
            param_3dcartapi_api_key: None,
            amazon_sp_client_id: None,
            amazon_sp_client_secret: None,
            amazon_sp_refresh_token: None,
            amazon_sp_aws_region: None,
            amazon_sp_api_environment: None,
            amazon_seller_id: None,
            aspdotnetstorefront_api_user: None,
            aspdotnetstorefront_api_pass: None,
            bigcommerceapi_admin_account: None,
            bigcommerceapi_api_path: None,
            bigcommerceapi_api_key: None,
            bigcommerceapi_client_id: None,
            bigcommerceapi_access_token: None,
            bigcommerceapi_context: None,
            bol_api_key: None,
            bol_api_secret: None,
            bol_retailer_id: None,
            demandware_client_id: None,
            demandware_api_password: None,
            demandware_user_name: None,
            demandware_user_password: None,
            ebay_client_id: None,
            ebay_client_secret: None,
            ebay_runame: None,
            ebay_access_token: None,
            ebay_refresh_token: None,
            ebay_environment: None,
            ebay_site_id: None,
            walmart_client_id: None,
            walmart_client_secret: None,
            walmart_environment: None,
            walmart_channel_type: None,
            walmart_region: None,
            ecwid_acess_token: None,
            ecwid_store_id: None,
            lazada_app_id: None,
            lazada_app_secret: None,
            lazada_refresh_token: None,
            lazada_region: None,
            lightspeed_api_key: None,
            lightspeed_api_secret: None,
            etsy_keystring: None,
            etsy_shared_secret: None,
            etsy_access_token: None,
            etsy_token_secret: None,
            etsy_client_id: None,
            etsy_refresh_token: None,
            facebook_app_id: None,
            facebook_app_secret: None,
            facebook_access_token: None,
            facebook_business_id: None,
            neto_api_key: None,
            neto_api_username: None,
            shopline_access_token: None,
            shopline_app_key: None,
            shopline_app_secret: None,
            shopify_access_token: None,
            shopify_api_key: None,
            shopify_api_password: None,
            shopify_shared_secret: None,
            shoplazza_access_token: None,
            shoplazza_shared_secret: None,
            shopware_access_key: None,
            shopware_api_key: None,
            shopware_api_secret: None,
            miva_access_token: None,
            miva_signature: None,
            tiendanube_user_id: None,
            tiendanube_access_token: None,
            tiendanube_client_secret: None,
            volusion_login: None,
            volusion_password: None,
            hybris_client_id: None,
            hybris_client_secret: None,
            hybris_username: None,
            hybris_password: None,
            hybris_websites: None,
            square_client_id: None,
            square_client_secret: None,
            square_refresh_token: None,
            squarespace_api_key: None,
            squarespace_client_id: None,
            squarespace_client_secret: None,
            squarespace_access_token: None,
            squarespace_refresh_token: None,
            commercehq_api_key: None,
            commercehq_api_password: None,
            wc_consumer_key: None,
            wc_consumer_secret: None,
            magento_consumer_key: None,
            magento_consumer_secret: None,
            magento_access_token: None,
            magento_token_secret: None,
            prestashop_webservice_key: None,
            wix_app_id,
            wix_app_secret_key,
            wix_instance_id: None,
            wix_refresh_token: None,
            mercado_libre_app_id: None,
            mercado_libre_app_secret_key: None,
            mercado_libre_refresh_token: None,
            zid_client_id: None,
            zid_client_secret: None,
            zid_access_token: None,
            zid_authorization: None,
            zid_refresh_token: None,
            flipkart_client_id: None,
            flipkart_client_secret: None,
            allegro_client_id: None,
            allegro_client_secret: None,
            allegro_access_token: None,
            allegro_refresh_token: None,
            allegro_environment: None,
            zoho_client_id: None,
            zoho_client_secret: None,
            zoho_refresh_token: None,
            zoho_region: None,
            otto_client_id: None,
            otto_client_secret: None,
            otto_app_id: None,
            otto_refresh_token: None,
            otto_environment: None,
            otto_access_token: None,
            tiktokshop_app_key: None,
            tiktokshop_app_secret: None,
            tiktokshop_refresh_token: None,
            tiktokshop_access_token: None,
            salla_client_id: None,
            salla_client_secret: None,
            salla_refresh_token: None,
            salla_access_token: None,
        }
    }
}
/// Store’s identifier which you can get from cart_list method
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CartId {
    #[serde(rename = "3DCart")]
    Variant3DCart,
    #[serde(rename = "3DCartApi")]
    Variant3DCartApi,
    #[serde(rename = "AceShop")]
    AceShop,
    #[serde(rename = "AmazonSP")]
    AmazonSp,
    #[serde(rename = "AspDotNetStorefront")]
    AspDotNetStorefront,
    #[serde(rename = "BigcommerceApi")]
    BigcommerceApi,
    #[serde(rename = "Bol")]
    Bol,
    #[serde(rename = "CommerceHQ")]
    CommerceHq,
    #[serde(rename = "Creloaded")]
    Creloaded,
    #[serde(rename = "Cscart")]
    Cscart,
    #[serde(rename = "Cubecart")]
    Cubecart,
    #[serde(rename = "Demandware")]
    Demandware,
    #[serde(rename = "EBay")]
    EBay,
    #[serde(rename = "Ecwid")]
    Ecwid,
    #[serde(rename = "EtsyAPIv3")]
    EtsyApiv3,
    #[serde(rename = "Flipkart")]
    Flipkart,
    #[serde(rename = "Gambio")]
    Gambio,
    #[serde(rename = "Hybris")]
    Hybris,
    #[serde(rename = "JooCart")]
    JooCart,
    #[serde(rename = "Lazada")]
    Lazada,
    #[serde(rename = "LightSpeed")]
    LightSpeed,
    #[serde(rename = "Magento1212")]
    Magento1212,
    #[serde(rename = "Magento2Api")]
    Magento2Api,
    #[serde(rename = "MercadoLibre")]
    MercadoLibre,
    #[serde(rename = "MijoShop")]
    MijoShop,
    #[serde(rename = "Miva")]
    Miva,
    #[serde(rename = "Neto")]
    Neto,
    #[serde(rename = "Opencart14")]
    Opencart14,
    #[serde(rename = "Oscmax2")]
    Oscmax2,
    #[serde(rename = "Oscommerce22ms2")]
    Oscommerce22ms2,
    #[serde(rename = "Otto")]
    Otto,
    #[serde(rename = "Oxid")]
    Oxid,
    #[serde(rename = "Pinnacle")]
    Pinnacle,
    #[serde(rename = "Prestashop")]
    Prestashop,
    #[serde(rename = "PrestashopApi")]
    PrestashopApi,
    #[serde(rename = "SSPremium")]
    SsPremium,
    #[serde(rename = "Salla")]
    Salla,
    #[serde(rename = "Shopify")]
    Shopify,
    #[serde(rename = "Shoplazza")]
    Shoplazza,
    #[serde(rename = "Shopline")]
    Shopline,
    #[serde(rename = "Shopware")]
    Shopware,
    #[serde(rename = "ShopwareApi")]
    ShopwareApi,
    #[serde(rename = "Square")]
    Square,
    #[serde(rename = "Squarespace")]
    Squarespace,
    #[serde(rename = "Tiendanube")]
    Tiendanube,
    #[serde(rename = "TikTokShop")]
    TikTokShop,
    #[serde(rename = "Tomatocart")]
    Tomatocart,
    #[serde(rename = "Ubercart")]
    Ubercart,
    #[serde(rename = "Virtuemart")]
    Virtuemart,
    #[serde(rename = "Volusion")]
    Volusion,
    #[serde(rename = "WPecommerce")]
    WPecommerce,
    #[serde(rename = "Walmart")]
    Walmart,
    #[serde(rename = "WebAsyst")]
    WebAsyst,
    #[serde(rename = "Wix")]
    Wix,
    #[serde(rename = "Woocommerce")]
    Woocommerce,
    #[serde(rename = "WoocommerceApi")]
    WoocommerceApi,
    #[serde(rename = "Xcart")]
    Xcart,
    #[serde(rename = "Xtcommerce")]
    Xtcommerce,
    #[serde(rename = "XtcommerceVeyton")]
    XtcommerceVeyton,
    #[serde(rename = "Zencart137")]
    Zencart137,
    #[serde(rename = "Zid")]
    Zid,
    #[serde(rename = "Zoey")]
    Zoey,
    #[serde(rename = "Zoho")]
    Zoho,
}

impl Default for CartId {
    fn default() -> CartId {
        Self::Variant3DCart
    }
}


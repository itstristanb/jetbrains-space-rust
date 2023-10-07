# EsApp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**owner** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**owner_app** | Option<[**crate::models::EsApp**](ES_App.md)> |  | [optional]
**client_id** | **String** |  | 
**name** | **String** |  | 
**email** | Option<**String**> |  | [optional]
**picture** | Option<**String**> |  | [optional]
**default_external_picture** | Option<**String**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**kind** | Option<**String**> |  | [optional]
**presentable_name** | Option<**String**> |  | [optional]
**application_type** | Option<[**crate::models::ApplicationType**](ApplicationType.md)> |  | [optional]
**client_credentials_flow_enabled** | Option<**bool**> |  | [optional]
**code_flow_enabled** | Option<**bool**> |  | [optional]
**code_flow_redirect_uris** | Option<**String**> |  | [optional]
**pkce_required** | Option<**bool**> |  | [optional]
**implicit_flow_enabled** | Option<**bool**> |  | [optional]
**implicit_flow_redirect_uris** | Option<**String**> |  | [optional]
**endpoint_uri** | Option<**String**> |  | [optional]
**has_verification_token** | Option<**bool**> |  | [optional]
**has_signing_key** | Option<**bool**> |  | [optional]
**has_public_key_signature** | Option<**bool**> |  | [optional]
**endpoint_ssl_verification** | Option<**bool**> |  | [optional]
**basic_auth_username** | Option<**String**> |  | [optional]
**has_bearer_token** | Option<**bool**> |  | [optional]
**ssl_keystore_auth** | Option<**String**> |  | [optional]
**archived** | **bool** |  | 
**description** | Option<**String**> |  | [optional]
**metadata** | Option<[**crate::models::ApplicationMetadata**](ApplicationMetadata.md)> |  | [optional]
**settings** | [**crate::models::EsAppSettings**](ES_AppSettings.md) |  | 
**error_message** | Option<**String**> |  | [optional]
**domains** | [**Vec<crate::models::ApplicationUnfurlDomain>**](ApplicationUnfurlDomain.md) |  | 
**patterns** | [**Vec<crate::models::ApplicationUnfurlPattern>**](ApplicationUnfurlPattern.md) |  | 
**contexts** | [**Vec<crate::models::AppUiExtContextData>**](AppUiExtContextData.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



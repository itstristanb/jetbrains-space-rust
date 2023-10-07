# ApplicationsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**picture_attachment_id** | Option<**String**> |  | [optional]
**default_external_picture** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**client_id** | Option<**String**> |  | [optional]
**client_secret** | Option<**String**> |  | [optional]
**client_credentials_flow_enabled** | Option<**bool**> |  | [optional][default to true]
**code_flow_enabled** | Option<**bool**> |  | [optional][default to false]
**code_flow_redirect_uris** | Option<**String**> |  | [optional]
**pkce_required** | Option<**bool**> |  | [optional]
**public_clients_allowed** | Option<**bool**> |  | [optional]
**implicit_flow_enabled** | Option<**bool**> |  | [optional][default to false]
**implicit_flow_redirect_uris** | Option<**String**> |  | [optional]
**endpoint_uri** | Option<**String**> |  | [optional]
**endpoint_ssl_verification** | Option<**bool**> |  | [optional]
**app_level_auth** | Option<[**crate::models::EndpointAuthCreate**](EndpointAuthCreate.md)> |  | [optional]
**ssl_keystore_auth** | Option<**String**> |  | [optional]
**has_signing_key** | Option<**bool**> |  | [optional]
**has_public_key_signature** | Option<**bool**> |  | [optional]
**basic_auth_username** | Option<**String**> |  | [optional]
**basic_auth_password** | Option<**String**> |  | [optional]
**bearer_auth_token** | Option<**String**> |  | [optional]
**connect_to_space** | Option<**bool**> |  | [optional][default to false]
**state** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



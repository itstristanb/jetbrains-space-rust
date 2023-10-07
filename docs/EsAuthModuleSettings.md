# EsAuthModuleSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**password_module** | Option<**bool**> |  | [optional]
**federated_module** | Option<**bool**> |  | [optional]
**password_strength_policy** | [**crate::models::PasswordStrength**](PasswordStrength.md) |  | 
**domains** | **Vec<String>** |  | 
**r#type** | [**crate::models::LdapModuleType**](LdapModuleType.md) |  | 
**register_new_users** | **bool** |  | 
**server_url** | **String** |  | 
**connection_timeout** | **i32** |  | 
**read_timeout** | **i32** |  | 
**ssl_keystore** | Option<[**crate::models::SslKeystore**](SSLKeystore.md)> |  | [optional]
**team_mappings** | [**Vec<crate::models::EsTeamMapping>**](ES_TeamMapping.md) |  | 
**referral_ignored** | **bool** |  | 
**filter** | **String** |  | 
**bind_user_dn** | **String** |  | 
**bind_user_password** | **String** |  | 
**attribute_names** | [**crate::models::EsSamlAttributeNames**](ES_SamlAttributeNames.md) |  | 
**register_new_user_rules** | Option<[**Vec<crate::models::GoogleRegisterNewUserRule>**](GoogleRegisterNewUserRule.md)> |  | [optional]
**idp_url** | **String** |  | 
**idp_entity_id** | **String** |  | 
**idp_certificate_sha256** | **String** |  | 
**sp_entity_id** | **String** |  | 
**contact_profile_id** | Option<**String**> |  | [optional]
**github_url** | **String** |  | 
**client_id** | **String** |  | 
**client_secret** | **String** |  | 
**organizations** | **Vec<String>** |  | 
**discovery_url** | Option<**String**> |  | [optional]
**issuer** | **String** |  | 
**authorization_endpoint** | **String** |  | 
**token_endpoint** | **String** |  | 
**token_keys_endpoint** | **String** |  | 
**user_info_endpoint** | **String** |  | 
**tenant_id** | **String** |  | 
**email_verified** | **bool** |  | 
**hub_url** | **String** |  | 
**org_auth_provider_name** | Option<**String**> |  | [optional]
**groups** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



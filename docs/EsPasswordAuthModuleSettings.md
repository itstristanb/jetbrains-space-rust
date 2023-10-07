# EsPasswordAuthModuleSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**password_strength_policy** | [**crate::models::PasswordStrength**](PasswordStrength.md) |  | 
**domains** | Option<**Vec<String>**> |  | [optional]
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
**attribute_names** | [**crate::models::EsLdapAttributeNames**](ES_LdapAttributeNames.md) |  | 
**register_new_user_rules** | Option<[**Vec<crate::models::LdapRegisterNewUserRule>**](LdapRegisterNewUserRule.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



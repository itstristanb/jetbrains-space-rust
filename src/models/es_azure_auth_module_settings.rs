/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EsAzureAuthModuleSettings {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "registerNewUsers")]
    pub register_new_users: bool,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "registerNewUserRules", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub register_new_user_rules: Option<Option<Vec<crate::models::AzureRegisterNewUserRule>>>,
}

impl EsAzureAuthModuleSettings {
    pub fn new(tenant_id: String, client_id: String, client_secret: String, register_new_users: bool, email_verified: bool) -> EsAzureAuthModuleSettings {
        EsAzureAuthModuleSettings {
            tenant_id,
            client_id,
            client_secret,
            register_new_users,
            email_verified,
            register_new_user_rules: None,
        }
    }
}


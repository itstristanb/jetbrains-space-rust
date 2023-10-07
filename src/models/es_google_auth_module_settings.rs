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
pub struct EsGoogleAuthModuleSettings {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "registerNewUsers")]
    pub register_new_users: bool,
    #[serde(rename = "domains")]
    pub domains: Vec<String>,
    #[serde(rename = "registerNewUserRules", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub register_new_user_rules: Option<Option<Vec<crate::models::GoogleRegisterNewUserRule>>>,
}

impl EsGoogleAuthModuleSettings {
    pub fn new(client_id: String, client_secret: String, register_new_users: bool, domains: Vec<String>) -> EsGoogleAuthModuleSettings {
        EsGoogleAuthModuleSettings {
            client_id,
            client_secret,
            register_new_users,
            domains,
            register_new_user_rules: None,
        }
    }
}


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
pub struct AuthModulesIdPatchRequest {
    #[serde(rename = "key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub key: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "enabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Option<bool>>,
    #[serde(rename = "iconDataURI", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_data_uri: Option<Option<String>>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::EsAuthModuleSettings>>,
}

impl AuthModulesIdPatchRequest {
    pub fn new() -> AuthModulesIdPatchRequest {
        AuthModulesIdPatchRequest {
            key: None,
            name: None,
            enabled: None,
            icon_data_uri: None,
            settings: None,
        }
    }
}



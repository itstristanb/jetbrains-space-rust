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
pub struct EsPersonalToken {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "apiScope")]
    pub api_scope: Box<crate::models::XScopeApi>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "expires", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires: Option<Option<String>>,
    #[serde(rename = "lastAccess", skip_serializing_if = "Option::is_none")]
    pub last_access: Option<Box<crate::models::AccessRecord>>,
}

impl EsPersonalToken {
    pub fn new(id: String, name: String, profile: crate::models::TdMemberProfile, scope: String, api_scope: crate::models::XScopeApi, created: String) -> EsPersonalToken {
        EsPersonalToken {
            id,
            name,
            profile: Box::new(profile),
            scope,
            api_scope: Box::new(api_scope),
            created,
            expires: None,
            last_access: None,
        }
    }
}


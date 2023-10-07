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
pub struct EsApplicationPassword {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "scope")]
    pub scope: Box<crate::models::XScopeApi>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "lastAccess", skip_serializing_if = "Option::is_none")]
    pub last_access: Option<Box<crate::models::AccessRecord>>,
}

impl EsApplicationPassword {
    pub fn new(id: String, profile: crate::models::TdMemberProfile, name: String, scope: crate::models::XScopeApi, created: String) -> EsApplicationPassword {
        EsApplicationPassword {
            id,
            profile: Box::new(profile),
            name,
            scope: Box::new(scope),
            created,
            last_access: None,
        }
    }
}



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
pub struct EsAuthenticationSession {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "expires")]
    pub expires: String,
    #[serde(rename = "lastAccess", skip_serializing_if = "Option::is_none")]
    pub last_access: Option<Box<crate::models::AccessRecord>>,
    #[serde(rename = "current")]
    pub current: bool,
}

impl EsAuthenticationSession {
    pub fn new(id: String, profile: crate::models::TdMemberProfile, created: String, expires: String, current: bool) -> EsAuthenticationSession {
        EsAuthenticationSession {
            id,
            profile: Box::new(profile),
            created,
            expires,
            last_access: None,
            current,
        }
    }
}



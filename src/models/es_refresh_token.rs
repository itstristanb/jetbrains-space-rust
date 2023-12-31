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
pub struct EsRefreshToken {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "scope")]
    pub scope: Box<crate::models::XScopeApi>,
    #[serde(rename = "lastAccess", skip_serializing_if = "Option::is_none")]
    pub last_access: Option<Box<crate::models::AccessRecord>>,
}

impl EsRefreshToken {
    pub fn new(id: String, scope: crate::models::XScopeApi) -> EsRefreshToken {
        EsRefreshToken {
            id,
            scope: Box::new(scope),
            last_access: None,
        }
    }
}



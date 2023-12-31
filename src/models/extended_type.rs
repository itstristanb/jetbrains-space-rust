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
pub struct ExtendedType {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "apiClassName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_class_name: Option<Option<String>>,
    #[serde(rename = "scopeType")]
    pub scope_type: crate::models::ExtendedTypeScopeType,
}

impl ExtendedType {
    pub fn new(key: String, display_name: String, scope_type: crate::models::ExtendedTypeScopeType) -> ExtendedType {
        ExtendedType {
            key,
            display_name,
            api_class_name: None,
            scope_type,
        }
    }
}



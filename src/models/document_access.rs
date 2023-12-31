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
pub struct DocumentAccess {
    #[serde(rename = "permissions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<Vec<crate::models::DocumentAccessRecipient>>>,
    #[serde(rename = "inherited", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub inherited: Option<Option<Vec<crate::models::DocumentAccessRecipient>>>,
}

impl DocumentAccess {
    pub fn new() -> DocumentAccess {
        DocumentAccess {
            permissions: None,
            inherited: None,
        }
    }
}



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
pub struct RightDto {
    #[serde(rename = "rightCode")]
    pub right_code: Box<crate::models::PermissionIdentifier>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "group")]
    pub group: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "targetName")]
    pub target_name: String,
    #[serde(rename = "editable")]
    pub editable: bool,
    #[serde(rename = "status")]
    pub status: crate::models::RightStatus,
    #[serde(rename = "modificationAuthor", skip_serializing_if = "Option::is_none")]
    pub modification_author: Option<Box<crate::models::CPrincipal>>,
    #[serde(rename = "modificationTimestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub modification_timestamp: Option<Option<String>>,
    #[serde(rename = "dependencies")]
    pub dependencies: Vec<String>,
}

impl RightDto {
    pub fn new(right_code: crate::models::PermissionIdentifier, name: String, group: String, description: String, target_name: String, editable: bool, status: crate::models::RightStatus, dependencies: Vec<String>) -> RightDto {
        RightDto {
            right_code: Box::new(right_code),
            name,
            group,
            description,
            target_name,
            editable,
            status,
            modification_author: None,
            modification_timestamp: None,
            dependencies,
        }
    }
}



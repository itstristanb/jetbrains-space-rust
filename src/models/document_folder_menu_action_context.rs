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
pub struct DocumentFolderMenuActionContext {
    #[serde(rename = "folderIdentifier")]
    pub folder_identifier: Box<crate::models::FolderIdentifier>,
    #[serde(rename = "projectIdentifier", skip_serializing_if = "Option::is_none")]
    pub project_identifier: Option<Box<crate::models::ProjectIdentifier>>,
}

impl DocumentFolderMenuActionContext {
    pub fn new(folder_identifier: crate::models::FolderIdentifier) -> DocumentFolderMenuActionContext {
        DocumentFolderMenuActionContext {
            folder_identifier: Box::new(folder_identifier),
            project_identifier: None,
        }
    }
}



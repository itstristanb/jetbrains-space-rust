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
pub struct ProjectsProjectDocumentsFoldersPostRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentFolder")]
    pub parent_folder: Box<crate::models::FolderIdentifier>,
}

impl ProjectsProjectDocumentsFoldersPostRequest {
    pub fn new(name: String, parent_folder: crate::models::FolderIdentifier) -> ProjectsProjectDocumentsFoldersPostRequest {
        ProjectsProjectDocumentsFoldersPostRequest {
            name,
            parent_folder: Box::new(parent_folder),
        }
    }
}



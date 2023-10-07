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
pub struct ProjectsProjectDocumentsDocumentIdCopyPostRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "folder")]
    pub folder: Box<crate::models::FolderIdentifier>,
}

impl ProjectsProjectDocumentsDocumentIdCopyPostRequest {
    pub fn new(name: String, folder: crate::models::FolderIdentifier) -> ProjectsProjectDocumentsDocumentIdCopyPostRequest {
        ProjectsProjectDocumentsDocumentIdCopyPostRequest {
            name,
            folder: Box::new(folder),
        }
    }
}



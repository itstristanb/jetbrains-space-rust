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
pub struct DocumentFolderWithChildren {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "containerLinkId")]
    pub container_link_id: String,
    #[serde(rename = "containerInfo")]
    pub container_info: Box<crate::models::DocumentContainerInfo>,
    #[serde(rename = "subfolders")]
    pub subfolders: Vec<crate::models::DocumentFolderWithChildren>,
    #[serde(rename = "documents")]
    pub documents: Vec<crate::models::Document>,
}

impl DocumentFolderWithChildren {
    pub fn new(id: String, archived: bool, container_link_id: String, container_info: crate::models::DocumentContainerInfo, subfolders: Vec<crate::models::DocumentFolderWithChildren>, documents: Vec<crate::models::Document>) -> DocumentFolderWithChildren {
        DocumentFolderWithChildren {
            id,
            archived,
            container_link_id,
            container_info: Box::new(container_info),
            subfolders,
            documents,
        }
    }
}


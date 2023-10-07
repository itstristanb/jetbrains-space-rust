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
pub struct DocumentFolderItemJbsFolderItem {
    #[serde(rename = "folder")]
    pub folder: Box<crate::models::DocumentFolder>,
    #[serde(rename = "highlights", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Box<crate::models::DocumentsSearchHighlights>>,
}

impl DocumentFolderItemJbsFolderItem {
    pub fn new(folder: crate::models::DocumentFolder) -> DocumentFolderItemJbsFolderItem {
        DocumentFolderItemJbsFolderItem {
            folder: Box::new(folder),
            highlights: None,
        }
    }
}


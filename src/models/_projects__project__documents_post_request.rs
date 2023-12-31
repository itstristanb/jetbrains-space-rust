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
pub struct ProjectsProjectDocumentsPostRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "folder")]
    pub folder: Box<crate::models::FolderIdentifier>,
    #[serde(rename = "bodyIn")]
    pub body_in: Box<crate::models::DocumentBodyCreateIn>,
    #[serde(rename = "publicationDetailsIn", skip_serializing_if = "Option::is_none")]
    pub publication_details_in: Option<Box<crate::models::BlogPublicationDetailsIn>>,
}

impl ProjectsProjectDocumentsPostRequest {
    pub fn new(name: String, folder: crate::models::FolderIdentifier, body_in: crate::models::DocumentBodyCreateIn) -> ProjectsProjectDocumentsPostRequest {
        ProjectsProjectDocumentsPostRequest {
            name,
            folder: Box::new(folder),
            body_in: Box::new(body_in),
            publication_details_in: None,
        }
    }
}



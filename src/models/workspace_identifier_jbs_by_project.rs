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
pub struct WorkspaceIdentifierJbsByProject {
    #[serde(rename = "project")]
    pub project: Box<crate::models::ProjectIdentifier>,
    #[serde(rename = "number")]
    pub number: i32,
}

impl WorkspaceIdentifierJbsByProject {
    pub fn new(project: crate::models::ProjectIdentifier, number: i32) -> WorkspaceIdentifierJbsByProject {
        WorkspaceIdentifierJbsByProject {
            project: Box::new(project),
            number,
        }
    }
}



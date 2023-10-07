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
pub struct DocumentCfParametersInput {
    #[serde(rename = "documentScope")]
    pub document_scope: Box<crate::models::DocumentCfScopeInputJbsProject>,
}

impl DocumentCfParametersInput {
    pub fn new(document_scope: crate::models::DocumentCfScopeInputJbsProject) -> DocumentCfParametersInput {
        DocumentCfParametersInput {
            document_scope: Box::new(document_scope),
        }
    }
}


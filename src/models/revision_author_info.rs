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
pub struct RevisionAuthorInfo {
    #[serde(rename = "revisionInfo")]
    pub revision_info: Box<crate::models::RevisionInfo>,
    #[serde(rename = "author")]
    pub author: Box<crate::models::RevisionAuthor>,
}

impl RevisionAuthorInfo {
    pub fn new(revision_info: crate::models::RevisionInfo, author: crate::models::RevisionAuthor) -> RevisionAuthorInfo {
        RevisionAuthorInfo {
            revision_info: Box::new(revision_info),
            author: Box::new(author),
        }
    }
}



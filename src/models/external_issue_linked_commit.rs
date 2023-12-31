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
pub struct ExternalIssueLinkedCommit {
    #[serde(rename = "commit")]
    pub commit: Box<crate::models::GitCommitInfo>,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Box<crate::models::GitCommitChanges>>,
    #[serde(rename = "url")]
    pub url: String,
}

impl ExternalIssueLinkedCommit {
    pub fn new(commit: crate::models::GitCommitInfo, url: String) -> ExternalIssueLinkedCommit {
        ExternalIssueLinkedCommit {
            commit: Box::new(commit),
            changes: None,
            url,
        }
    }
}



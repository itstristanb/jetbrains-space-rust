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
pub struct CommitLinkedExternalIssuesChanged {
    #[serde(rename = "project")]
    pub project: Box<crate::models::PrProject>,
    #[serde(rename = "repositoryId")]
    pub repository_id: String,
    #[serde(rename = "repositoryName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<Option<String>>,
    #[serde(rename = "commitId")]
    pub commit_id: String,
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<Box<crate::models::GitCommitInfo>>,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Box<crate::models::GitCommitChanges>>,
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    #[serde(rename = "issues")]
    pub issues: Vec<crate::models::ExternalIssueIdOut>,
}

impl CommitLinkedExternalIssuesChanged {
    pub fn new(project: crate::models::PrProject, repository_id: String, commit_id: String, issues: Vec<crate::models::ExternalIssueIdOut>) -> CommitLinkedExternalIssuesChanged {
        CommitLinkedExternalIssuesChanged {
            project: Box::new(project),
            repository_id,
            repository_name: None,
            commit_id,
            commit: None,
            changes: None,
            url: None,
            issues,
        }
    }
}


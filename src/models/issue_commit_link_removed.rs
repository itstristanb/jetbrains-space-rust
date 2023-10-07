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
pub struct IssueCommitLinkRemoved {
    #[serde(rename = "project")]
    pub project: Box<crate::models::PrProject>,
    #[serde(rename = "repositoryId")]
    pub repository_id: String,
    #[serde(rename = "commitId")]
    pub commit_id: String,
    #[serde(rename = "issuePrefix")]
    pub issue_prefix: String,
    #[serde(rename = "issueId")]
    pub issue_id: String,
}

impl IssueCommitLinkRemoved {
    pub fn new(project: crate::models::PrProject, repository_id: String, commit_id: String, issue_prefix: String, issue_id: String) -> IssueCommitLinkRemoved {
        IssueCommitLinkRemoved {
            project: Box::new(project),
            repository_id,
            commit_id,
            issue_prefix,
            issue_id,
        }
    }
}



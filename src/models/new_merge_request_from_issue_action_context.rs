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
pub struct NewMergeRequestFromIssueActionContext {
    #[serde(rename = "projectKey")]
    pub project_key: Box<crate::models::ProjectKey>,
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "commitId")]
    pub commit_id: String,
    #[serde(rename = "issueNumber")]
    pub issue_number: i32,
    #[serde(rename = "projectRepos")]
    pub project_repos: Box<crate::models::ProjectReposRecord>,
    #[serde(rename = "issueCommitsRef")]
    pub issue_commits_ref: Box<crate::models::IssueCommits>,
}

impl NewMergeRequestFromIssueActionContext {
    pub fn new(project_key: crate::models::ProjectKey, repository: String, commit_id: String, issue_number: i32, project_repos: crate::models::ProjectReposRecord, issue_commits_ref: crate::models::IssueCommits) -> NewMergeRequestFromIssueActionContext {
        NewMergeRequestFromIssueActionContext {
            project_key: Box::new(project_key),
            repository,
            commit_id,
            issue_number,
            project_repos: Box::new(project_repos),
            issue_commits_ref: Box::new(issue_commits_ref),
        }
    }
}



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
pub struct NewMergeRequestFromIssueBranchActionContext {
    #[serde(rename = "projectKey")]
    pub project_key: Box<crate::models::ProjectKey>,
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "branchHead")]
    pub branch_head: String,
    #[serde(rename = "issueNumber")]
    pub issue_number: i32,
    #[serde(rename = "projectRepos")]
    pub project_repos: Box<crate::models::ProjectReposRecord>,
    #[serde(rename = "issueBranchesRef")]
    pub issue_branches_ref: Box<crate::models::IssueBranches>,
}

impl NewMergeRequestFromIssueBranchActionContext {
    pub fn new(project_key: crate::models::ProjectKey, repository: String, branch_head: String, issue_number: i32, project_repos: crate::models::ProjectReposRecord, issue_branches_ref: crate::models::IssueBranches) -> NewMergeRequestFromIssueBranchActionContext {
        NewMergeRequestFromIssueBranchActionContext {
            project_key: Box::new(project_key),
            repository,
            branch_head,
            issue_number,
            project_repos: Box::new(project_repos),
            issue_branches_ref: Box::new(issue_branches_ref),
        }
    }
}



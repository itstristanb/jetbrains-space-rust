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
pub struct ProjectsProjectCodeReviewsMergeRequestsPostRequest {
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "sourceBranch")]
    pub source_branch: String,
    #[serde(rename = "targetBranch")]
    pub target_branch: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "reviewers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Option<Vec<crate::models::ReviewerParam>>>,
}

impl ProjectsProjectCodeReviewsMergeRequestsPostRequest {
    pub fn new(repository: String, source_branch: String, target_branch: String, title: String) -> ProjectsProjectCodeReviewsMergeRequestsPostRequest {
        ProjectsProjectCodeReviewsMergeRequestsPostRequest {
            repository,
            source_branch,
            target_branch,
            title,
            description: None,
            reviewers: None,
        }
    }
}



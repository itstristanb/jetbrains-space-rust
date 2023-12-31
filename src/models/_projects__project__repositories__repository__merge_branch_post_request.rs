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
pub struct ProjectsProjectRepositoriesRepositoryMergeBranchPostRequest {
    #[serde(rename = "sourceBranch")]
    pub source_branch: String,
    #[serde(rename = "targetBranch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<Option<String>>,
    #[serde(rename = "commitMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<Option<String>>,
    #[serde(rename = "mergeMode")]
    pub merge_mode: crate::models::GitMergeMode,
}

impl ProjectsProjectRepositoriesRepositoryMergeBranchPostRequest {
    pub fn new(source_branch: String, merge_mode: crate::models::GitMergeMode) -> ProjectsProjectRepositoriesRepositoryMergeBranchPostRequest {
        ProjectsProjectRepositoriesRepositoryMergeBranchPostRequest {
            source_branch,
            target_branch: None,
            commit_message: None,
            merge_mode,
        }
    }
}



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
pub struct ProjectsProjectRepositoriesRepositoryCommitPostRequest {
    #[serde(rename = "baseCommit")]
    pub base_commit: String,
    #[serde(rename = "targetBranch")]
    pub target_branch: String,
    #[serde(rename = "commitMessage")]
    pub commit_message: String,
    #[serde(rename = "files")]
    pub files: Vec<crate::models::GitCommitFileRequest>,
}

impl ProjectsProjectRepositoriesRepositoryCommitPostRequest {
    pub fn new(base_commit: String, target_branch: String, commit_message: String, files: Vec<crate::models::GitCommitFileRequest>) -> ProjectsProjectRepositoriesRepositoryCommitPostRequest {
        ProjectsProjectRepositoriesRepositoryCommitPostRequest {
            base_commit,
            target_branch,
            commit_message,
            files,
        }
    }
}


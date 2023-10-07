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
pub struct SRepoCommitsWebhookEvent {
    #[serde(rename = "projectKey")]
    pub project_key: Box<crate::models::ProjectKey>,
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "commit")]
    pub commit: Box<crate::models::GitCommitInfoWithChanges>,
}

impl SRepoCommitsWebhookEvent {
    pub fn new(project_key: crate::models::ProjectKey, repository: String, commit: crate::models::GitCommitInfoWithChanges) -> SRepoCommitsWebhookEvent {
        SRepoCommitsWebhookEvent {
            project_key: Box::new(project_key),
            repository,
            commit: Box::new(commit),
        }
    }
}



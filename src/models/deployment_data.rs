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
pub struct DeploymentData {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "scheduledStart", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<Option<String>>,
    #[serde(rename = "startedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Option<String>>,
    #[serde(rename = "finishedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Option<String>>,
    #[serde(rename = "targetKey")]
    pub target_key: String,
    #[serde(rename = "status")]
    pub status: crate::models::DeploymentStatus,
    #[serde(rename = "syncStatus")]
    pub sync_status: crate::models::DeploymentSyncStatus,
    #[serde(rename = "externalLink", skip_serializing_if = "Option::is_none")]
    pub external_link: Option<Box<crate::models::ExternalLink>>,
    #[serde(rename = "commitRefs")]
    pub commit_refs: Vec<crate::models::DeploymentCommitRefDetails>,
}

impl DeploymentData {
    pub fn new(id: String, version: String, created_at: String, target_key: String, status: crate::models::DeploymentStatus, sync_status: crate::models::DeploymentSyncStatus, commit_refs: Vec<crate::models::DeploymentCommitRefDetails>) -> DeploymentData {
        DeploymentData {
            id,
            version,
            created_at,
            scheduled_start: None,
            started_at: None,
            finished_at: None,
            target_key,
            status,
            sync_status,
            external_link: None,
            commit_refs,
        }
    }
}



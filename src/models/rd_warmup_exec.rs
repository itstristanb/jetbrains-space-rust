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
pub struct RdWarmupExec {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "vcsData")]
    pub vcs_data: Box<crate::models::RdWarmupVcsData>,
    #[serde(rename = "status")]
    pub status: crate::models::WarmupExecutionStatus,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "finishedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Option<String>>,
    #[serde(rename = "ideType")]
    pub ide_type: Box<crate::models::IdeType>,
    #[serde(rename = "ideBuild")]
    pub ide_build: String,
    #[serde(rename = "ideVersion")]
    pub ide_version: String,
    #[serde(rename = "ideQuality", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ide_quality: Option<Option<String>>,
    #[serde(rename = "computeTaskId")]
    pub compute_task_id: String,
    #[serde(rename = "trigger")]
    pub trigger: Box<crate::models::WarmupExecutionTrigger>,
    #[serde(rename = "configurationSource")]
    pub configuration_source: Box<crate::models::RdConfigurationSource>,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl RdWarmupExec {
    pub fn new(id: String, project_id: String, vcs_data: crate::models::RdWarmupVcsData, status: crate::models::WarmupExecutionStatus, started_at: String, ide_type: crate::models::IdeType, ide_build: String, ide_version: String, compute_task_id: String, trigger: crate::models::WarmupExecutionTrigger, configuration_source: crate::models::RdConfigurationSource, archived: bool) -> RdWarmupExec {
        RdWarmupExec {
            id,
            project_id,
            vcs_data: Box::new(vcs_data),
            status,
            started_at,
            finished_at: None,
            ide_type: Box::new(ide_type),
            ide_build,
            ide_version,
            ide_quality: None,
            compute_task_id,
            trigger: Box::new(trigger),
            configuration_source: Box::new(configuration_source),
            archived,
        }
    }
}


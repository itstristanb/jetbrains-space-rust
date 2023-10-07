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
pub struct JobExecutionDto {
    #[serde(rename = "executionId")]
    pub execution_id: String,
    #[serde(rename = "executionNumber")]
    pub execution_number: i64,
    #[serde(rename = "jobId")]
    pub job_id: String,
    #[serde(rename = "jobName")]
    pub job_name: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "branch")]
    pub branch: String,
    #[serde(rename = "status")]
    pub status: crate::models::ExecutionStatus,
    #[serde(rename = "triggeredTime")]
    pub triggered_time: i64,
    #[serde(rename = "startedTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub started_time: Option<Option<i64>>,
    #[serde(rename = "finishedTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub finished_time: Option<Option<i64>>,
    #[serde(rename = "changesCount")]
    pub changes_count: i32,
    #[serde(rename = "failureConditions")]
    pub failure_conditions: Vec<crate::models::FailureConditionDto>,
    #[serde(rename = "commitId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<Option<String>>,
}

impl JobExecutionDto {
    pub fn new(execution_id: String, execution_number: i64, job_id: String, job_name: String, project_id: String, branch: String, status: crate::models::ExecutionStatus, triggered_time: i64, changes_count: i32, failure_conditions: Vec<crate::models::FailureConditionDto>) -> JobExecutionDto {
        JobExecutionDto {
            execution_id,
            execution_number,
            job_id,
            job_name,
            project_id,
            branch,
            status,
            triggered_time,
            started_time: None,
            finished_time: None,
            changes_count,
            failure_conditions,
            commit_id: None,
        }
    }
}


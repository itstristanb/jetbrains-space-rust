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
pub struct ParameterLastUsageDto {
    #[serde(rename = "jobExecutionId")]
    pub job_execution_id: String,
    #[serde(rename = "stepExecutionId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub step_execution_id: Option<Option<String>>,
    #[serde(rename = "jobName")]
    pub job_name: String,
    #[serde(rename = "usedAt")]
    pub used_at: String,
}

impl ParameterLastUsageDto {
    pub fn new(job_execution_id: String, job_name: String, used_at: String) -> ParameterLastUsageDto {
        ParameterLastUsageDto {
            job_execution_id,
            step_execution_id: None,
            job_name,
            used_at,
        }
    }
}



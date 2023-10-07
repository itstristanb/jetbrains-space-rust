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
pub struct ProjectsProjectPlanningIssuesSyncBatchGet200Response {
    #[serde(rename = "etag")]
    pub etag: String,
    #[serde(rename = "data")]
    pub data: Vec<crate::models::Issue>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

impl ProjectsProjectPlanningIssuesSyncBatchGet200Response {
    pub fn new(etag: String, data: Vec<crate::models::Issue>, has_more: bool) -> ProjectsProjectPlanningIssuesSyncBatchGet200Response {
        ProjectsProjectPlanningIssuesSyncBatchGet200Response {
            etag,
            data,
            has_more,
        }
    }
}


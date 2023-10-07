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
pub struct ProjectsProjectRepositoriesRepositoryHeadsGet200Response {
    #[serde(rename = "next")]
    pub next: String,
    #[serde(rename = "totalCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<Option<i32>>,
    #[serde(rename = "data")]
    pub data: Vec<crate::models::BranchInfo>,
}

impl ProjectsProjectRepositoriesRepositoryHeadsGet200Response {
    pub fn new(next: String, data: Vec<crate::models::BranchInfo>) -> ProjectsProjectRepositoriesRepositoryHeadsGet200Response {
        ProjectsProjectRepositoriesRepositoryHeadsGet200Response {
            next,
            total_count: None,
            data,
        }
    }
}



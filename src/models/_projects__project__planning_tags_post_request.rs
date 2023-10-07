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
pub struct ProjectsProjectPlanningTagsPostRequest {
    #[serde(rename = "parentTagId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_tag_id: Option<Option<String>>,
    #[serde(rename = "path")]
    pub path: Vec<String>,
}

impl ProjectsProjectPlanningTagsPostRequest {
    pub fn new(path: Vec<String>) -> ProjectsProjectPlanningTagsPostRequest {
        ProjectsProjectPlanningTagsPostRequest {
            parent_tag_id: None,
            path,
        }
    }
}


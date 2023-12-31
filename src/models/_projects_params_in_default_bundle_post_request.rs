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
pub struct ProjectsParamsInDefaultBundlePostRequest {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl ProjectsParamsInDefaultBundlePostRequest {
    pub fn new(project_id: String, key: String, value: String) -> ProjectsParamsInDefaultBundlePostRequest {
        ProjectsParamsInDefaultBundlePostRequest {
            project_id,
            key,
            value,
        }
    }
}



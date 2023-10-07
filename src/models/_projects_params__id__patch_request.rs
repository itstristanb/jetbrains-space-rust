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
pub struct ProjectsParamsIdPatchRequest {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl ProjectsParamsIdPatchRequest {
    pub fn new(value: String) -> ProjectsParamsIdPatchRequest {
        ProjectsParamsIdPatchRequest {
            value,
            description: None,
        }
    }
}


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
pub struct ProjectsProjectResponsibilitiesResponsibilityIdAssigneesProfileIdPostRequest {
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<String>>,
}

impl ProjectsProjectResponsibilitiesResponsibilityIdAssigneesProfileIdPostRequest {
    pub fn new() -> ProjectsProjectResponsibilitiesResponsibilityIdAssigneesProfileIdPostRequest {
        ProjectsProjectResponsibilitiesResponsibilityIdAssigneesProfileIdPostRequest {
            role: None,
        }
    }
}



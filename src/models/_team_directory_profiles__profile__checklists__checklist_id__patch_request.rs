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
pub struct TeamDirectoryProfilesProfileChecklistsChecklistIdPatchRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl TeamDirectoryProfilesProfileChecklistsChecklistIdPatchRequest {
    pub fn new() -> TeamDirectoryProfilesProfileChecklistsChecklistIdPatchRequest {
        TeamDirectoryProfilesProfileChecklistsChecklistIdPatchRequest {
            name: None,
            description: None,
        }
    }
}



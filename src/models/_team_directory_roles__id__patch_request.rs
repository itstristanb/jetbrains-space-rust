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
pub struct TeamDirectoryRolesIdPatchRequest {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "parentId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<String>>,
}

impl TeamDirectoryRolesIdPatchRequest {
    pub fn new() -> TeamDirectoryRolesIdPatchRequest {
        TeamDirectoryRolesIdPatchRequest {
            name: None,
            parent_id: None,
        }
    }
}



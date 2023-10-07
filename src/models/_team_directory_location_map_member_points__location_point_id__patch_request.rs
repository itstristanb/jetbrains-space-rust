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
pub struct TeamDirectoryLocationMapMemberPointsLocationPointIdPatchRequest {
    #[serde(rename = "x", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub x: Option<Option<i32>>,
    #[serde(rename = "y", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub y: Option<Option<i32>>,
}

impl TeamDirectoryLocationMapMemberPointsLocationPointIdPatchRequest {
    pub fn new() -> TeamDirectoryLocationMapMemberPointsLocationPointIdPatchRequest {
        TeamDirectoryLocationMapMemberPointsLocationPointIdPatchRequest {
            x: None,
            y: None,
        }
    }
}


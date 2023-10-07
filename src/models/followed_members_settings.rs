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
pub struct FollowedMembersSettings {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "members")]
    pub members: Vec<String>,
}

impl FollowedMembersSettings {
    pub fn new(enabled: bool, members: Vec<String>) -> FollowedMembersSettings {
        FollowedMembersSettings {
            enabled,
            members,
        }
    }
}



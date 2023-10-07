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
pub struct FeatureFlagInfo {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "status")]
    pub status: crate::models::FeatureFlagStatus,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "since", skip_serializing_if = "Option::is_none")]
    pub since: Option<Box<crate::models::FeatureFlagDate>>,
    #[serde(rename = "issueNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issue_number: Option<Option<i32>>,
}

impl FeatureFlagInfo {
    pub fn new(name: String, description: String, status: crate::models::FeatureFlagStatus, owner: String) -> FeatureFlagInfo {
        FeatureFlagInfo {
            name,
            description,
            status,
            owner,
            since: None,
            issue_number: None,
        }
    }
}



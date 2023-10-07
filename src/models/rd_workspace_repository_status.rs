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
pub struct RdWorkspaceRepositoryStatus {
    #[serde(rename = "commit")]
    pub commit: String,
    #[serde(rename = "branch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub branch: Option<Option<String>>,
    #[serde(rename = "containsNotPushedChanges")]
    pub contains_not_pushed_changes: bool,
}

impl RdWorkspaceRepositoryStatus {
    pub fn new(commit: String, contains_not_pushed_changes: bool) -> RdWorkspaceRepositoryStatus {
        RdWorkspaceRepositoryStatus {
            commit,
            branch: None,
            contains_not_pushed_changes,
        }
    }
}


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
pub struct SafeMerge {
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::SafeMergeState>,
    #[serde(rename = "mergeCommitId")]
    pub merge_commit_id: String,
    #[serde(rename = "checks")]
    pub checks: Vec<crate::models::SafeMergeCheck>,
    #[serde(rename = "mergeOptions")]
    pub merge_options: Box<crate::models::MergeSelectOptions>,
    #[serde(rename = "startedBy", skip_serializing_if = "Option::is_none")]
    pub started_by: Option<Box<crate::models::TdMemberProfile>>,
    #[serde(rename = "startedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Option<String>>,
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<i64>>,
    #[serde(rename = "attempts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Option<i32>>,
}

impl SafeMerge {
    pub fn new(merge_commit_id: String, checks: Vec<crate::models::SafeMergeCheck>, merge_options: crate::models::MergeSelectOptions) -> SafeMerge {
        SafeMerge {
            state: None,
            merge_commit_id,
            checks,
            merge_options: Box::new(merge_options),
            started_by: None,
            started_at: None,
            duration: None,
            attempts: None,
        }
    }
}



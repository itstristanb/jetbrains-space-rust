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
pub struct MergeRequestBranchPair {
    #[serde(rename = "repositoryId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<Option<String>>,
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "sourceBranch")]
    pub source_branch: String,
    #[serde(rename = "targetBranch")]
    pub target_branch: String,
    #[serde(rename = "sourceBranchRef")]
    pub source_branch_ref: String,
    #[serde(rename = "sourceBranchInfo", skip_serializing_if = "Option::is_none")]
    pub source_branch_info: Option<Box<crate::models::MergeRequestBranch>>,
    #[serde(rename = "targetBranchInfo", skip_serializing_if = "Option::is_none")]
    pub target_branch_info: Option<Box<crate::models::MergeRequestBranch>>,
    #[serde(rename = "isMerged", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_merged: Option<Option<bool>>,
    #[serde(rename = "isStale", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_stale: Option<Option<bool>>,
}

impl MergeRequestBranchPair {
    pub fn new(repository: String, source_branch: String, target_branch: String, source_branch_ref: String) -> MergeRequestBranchPair {
        MergeRequestBranchPair {
            repository_id: None,
            repository,
            source_branch,
            target_branch,
            source_branch_ref,
            source_branch_info: None,
            target_branch_info: None,
            is_merged: None,
            is_stale: None,
        }
    }
}


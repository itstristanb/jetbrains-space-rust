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
pub struct MergeRequestBranchRestoredEvent {
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "branch")]
    pub branch: String,
    #[serde(rename = "branchType")]
    pub branch_type: crate::models::MergeRequestBranchType,
}

impl MergeRequestBranchRestoredEvent {
    pub fn new(repository: String, branch: String, branch_type: crate::models::MergeRequestBranchType) -> MergeRequestBranchRestoredEvent {
        MergeRequestBranchRestoredEvent {
            repository,
            branch,
            branch_type,
        }
    }
}



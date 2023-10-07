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
pub struct RepoChanges {
    #[serde(rename = "heads")]
    pub heads: Vec<crate::models::RepoHeadsChange>,
    #[serde(rename = "commits")]
    pub commits: Vec<crate::models::GitCommitInfoWithChanges>,
    #[serde(rename = "totalNewCommits")]
    pub total_new_commits: i32,
}

impl RepoChanges {
    pub fn new(heads: Vec<crate::models::RepoHeadsChange>, commits: Vec<crate::models::GitCommitInfoWithChanges>, total_new_commits: i32) -> RepoChanges {
        RepoChanges {
            heads,
            commits,
            total_new_commits,
        }
    }
}


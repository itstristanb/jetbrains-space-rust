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
pub struct MergeRequestMergedEvent {
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "sourceBranch")]
    pub source_branch: String,
    #[serde(rename = "targetBranch")]
    pub target_branch: String,
}

impl MergeRequestMergedEvent {
    pub fn new(repository: String, source_branch: String, target_branch: String) -> MergeRequestMergedEvent {
        MergeRequestMergedEvent {
            repository,
            source_branch,
            target_branch,
        }
    }
}



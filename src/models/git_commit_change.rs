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
pub struct GitCommitChange {
    #[serde(rename = "changeType")]
    pub change_type: crate::models::GitCommitChangeType,
    #[serde(rename = "old", skip_serializing_if = "Option::is_none")]
    pub old: Option<Box<crate::models::GitFile>>,
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    pub new: Option<Box<crate::models::GitFile>>,
    #[serde(rename = "revision")]
    pub revision: String,
    #[serde(rename = "diffSize", skip_serializing_if = "Option::is_none")]
    pub diff_size: Option<Box<crate::models::GitDiffSize>>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "detached", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub detached: Option<Option<bool>>,
    #[serde(rename = "constituentCommits", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub constituent_commits: Option<Option<Vec<String>>>,
}

impl GitCommitChange {
    pub fn new(change_type: crate::models::GitCommitChangeType, revision: String) -> GitCommitChange {
        GitCommitChange {
            change_type,
            old: None,
            new: None,
            revision,
            diff_size: None,
            path: None,
            detached: None,
            constituent_commits: None,
        }
    }
}



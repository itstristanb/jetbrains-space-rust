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
pub struct GitCommitInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "authorDate")]
    pub author_date: i64,
    #[serde(rename = "commitDate")]
    pub commit_date: i64,
    #[serde(rename = "author")]
    pub author: Box<crate::models::GitAuthorInfo>,
    #[serde(rename = "authorProfile", skip_serializing_if = "Option::is_none")]
    pub author_profile: Option<Box<crate::models::TdMemberProfile>>,
    #[serde(rename = "committer")]
    pub committer: Box<crate::models::GitAuthorInfo>,
    #[serde(rename = "committerProfile", skip_serializing_if = "Option::is_none")]
    pub committer_profile: Option<Box<crate::models::TdMemberProfile>>,
    #[serde(rename = "parents")]
    pub parents: Vec<String>,
    #[serde(rename = "heads")]
    pub heads: Vec<String>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Box<crate::models::GitCommitSignature>>,
    #[serde(rename = "committerIsSpace", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub committer_is_space: Option<Option<bool>>,
}

impl GitCommitInfo {
    pub fn new(id: String, message: String, author_date: i64, commit_date: i64, author: crate::models::GitAuthorInfo, committer: crate::models::GitAuthorInfo, parents: Vec<String>, heads: Vec<String>) -> GitCommitInfo {
        GitCommitInfo {
            id,
            message,
            author_date,
            commit_date,
            author: Box::new(author),
            author_profile: None,
            committer: Box::new(committer),
            committer_profile: None,
            parents,
            heads,
            signature: None,
            committer_is_space: None,
        }
    }
}



/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GitCommitChangeType {
    #[serde(rename = "ADDED")]
    Added,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "MODIFIED")]
    Modified,

}

impl ToString for GitCommitChangeType {
    fn to_string(&self) -> String {
        match self {
            Self::Added => String::from("ADDED"),
            Self::Deleted => String::from("DELETED"),
            Self::Modified => String::from("MODIFIED"),
        }
    }
}

impl Default for GitCommitChangeType {
    fn default() -> GitCommitChangeType {
        Self::Added
    }
}





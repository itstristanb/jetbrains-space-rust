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
pub enum DiffLineType {
    #[serde(rename = "ADDED")]
    Added,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "MODIFIED")]
    Modified,
    #[serde(rename = "FILTERED_ADDED")]
    FilteredAdded,
    #[serde(rename = "FILTERED_DELETED")]
    FilteredDeleted,
    #[serde(rename = "FILTERED")]
    Filtered,
    #[serde(rename = "FILTERED_MODIFIED")]
    FilteredModified,
    #[serde(rename = "CONFLICT_OLD")]
    ConflictOld,
    #[serde(rename = "CONFLICT_NEW")]
    ConflictNew,

}

impl ToString for DiffLineType {
    fn to_string(&self) -> String {
        match self {
            Self::Added => String::from("ADDED"),
            Self::Deleted => String::from("DELETED"),
            Self::Modified => String::from("MODIFIED"),
            Self::FilteredAdded => String::from("FILTERED_ADDED"),
            Self::FilteredDeleted => String::from("FILTERED_DELETED"),
            Self::Filtered => String::from("FILTERED"),
            Self::FilteredModified => String::from("FILTERED_MODIFIED"),
            Self::ConflictOld => String::from("CONFLICT_OLD"),
            Self::ConflictNew => String::from("CONFLICT_NEW"),
        }
    }
}

impl Default for DiffLineType {
    fn default() -> DiffLineType {
        Self::Added
    }
}




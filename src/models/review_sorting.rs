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
pub enum ReviewSorting {
    #[serde(rename = "CreatedAtDesc")]
    CreatedAtDesc,
    #[serde(rename = "CreatedAtAsc")]
    CreatedAtAsc,
    #[serde(rename = "LastUpdatedDesc")]
    LastUpdatedDesc,
    #[serde(rename = "LastUpdatedAsc")]
    LastUpdatedAsc,

}

impl ToString for ReviewSorting {
    fn to_string(&self) -> String {
        match self {
            Self::CreatedAtDesc => String::from("CreatedAtDesc"),
            Self::CreatedAtAsc => String::from("CreatedAtAsc"),
            Self::LastUpdatedDesc => String::from("LastUpdatedDesc"),
            Self::LastUpdatedAsc => String::from("LastUpdatedAsc"),
        }
    }
}

impl Default for ReviewSorting {
    fn default() -> ReviewSorting {
        Self::CreatedAtDesc
    }
}





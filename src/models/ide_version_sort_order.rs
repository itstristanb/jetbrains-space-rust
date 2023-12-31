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
pub enum IdeVersionSortOrder {
    #[serde(rename = "ByReleaseDate")]
    ByReleaseDate,
    #[serde(rename = "ByVersionAndQuality")]
    ByVersionAndQuality,

}

impl ToString for IdeVersionSortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::ByReleaseDate => String::from("ByReleaseDate"),
            Self::ByVersionAndQuality => String::from("ByVersionAndQuality"),
        }
    }
}

impl Default for IdeVersionSortOrder {
    fn default() -> IdeVersionSortOrder {
        Self::ByReleaseDate
    }
}





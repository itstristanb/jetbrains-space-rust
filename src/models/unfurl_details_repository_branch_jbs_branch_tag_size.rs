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
pub enum UnfurlDetailsRepositoryBranchJbsBranchTagSize {
    #[serde(rename = "SMALL")]
    Small,
    #[serde(rename = "NORMAL")]
    Normal,

}

impl ToString for UnfurlDetailsRepositoryBranchJbsBranchTagSize {
    fn to_string(&self) -> String {
        match self {
            Self::Small => String::from("SMALL"),
            Self::Normal => String::from("NORMAL"),
        }
    }
}

impl Default for UnfurlDetailsRepositoryBranchJbsBranchTagSize {
    fn default() -> UnfurlDetailsRepositoryBranchJbsBranchTagSize {
        Self::Small
    }
}




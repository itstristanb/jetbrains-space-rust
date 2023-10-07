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
pub enum GitMergeMode {
    #[serde(rename = "FF")]
    Ff,
    #[serde(rename = "FF_ONLY")]
    FfOnly,
    #[serde(rename = "NO_FF")]
    NoFf,

}

impl ToString for GitMergeMode {
    fn to_string(&self) -> String {
        match self {
            Self::Ff => String::from("FF"),
            Self::FfOnly => String::from("FF_ONLY"),
            Self::NoFf => String::from("NO_FF"),
        }
    }
}

impl Default for GitMergeMode {
    fn default() -> GitMergeMode {
        Self::Ff
    }
}





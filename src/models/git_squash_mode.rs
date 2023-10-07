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
pub enum GitSquashMode {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "NONE")]
    None,

}

impl ToString for GitSquashMode {
    fn to_string(&self) -> String {
        match self {
            Self::All => String::from("ALL"),
            Self::Auto => String::from("AUTO"),
            Self::None => String::from("NONE"),
        }
    }
}

impl Default for GitSquashMode {
    fn default() -> GitSquashMode {
        Self::All
    }
}





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
pub enum VersionStatus {
    #[serde(rename = "Outdated")]
    Outdated,
    #[serde(rename = "Active")]
    Active,

}

impl ToString for VersionStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Outdated => String::from("Outdated"),
            Self::Active => String::from("Active"),
        }
    }
}

impl Default for VersionStatus {
    fn default() -> VersionStatus {
        Self::Outdated
    }
}





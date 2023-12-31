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
pub enum HierarchyRole {
    #[serde(rename = "SEALED")]
    Sealed,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "FINAL")]
    Final,
    #[serde(rename = "ABSTRACT")]
    Abstract,
    #[serde(rename = "INTERFACE")]
    Interface,

}

impl ToString for HierarchyRole {
    fn to_string(&self) -> String {
        match self {
            Self::Sealed => String::from("SEALED"),
            Self::Open => String::from("OPEN"),
            Self::Final => String::from("FINAL"),
            Self::Abstract => String::from("ABSTRACT"),
            Self::Interface => String::from("INTERFACE"),
        }
    }
}

impl Default for HierarchyRole {
    fn default() -> HierarchyRole {
        Self::Sealed
    }
}





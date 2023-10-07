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
pub enum PermissionRoleType {
    #[serde(rename = "PREDEFINED")]
    Predefined,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "AUTHORIZABLE")]
    Authorizable,
    #[serde(rename = "ANONYMOUS")]
    Anonymous,

}

impl ToString for PermissionRoleType {
    fn to_string(&self) -> String {
        match self {
            Self::Predefined => String::from("PREDEFINED"),
            Self::Custom => String::from("CUSTOM"),
            Self::Authorizable => String::from("AUTHORIZABLE"),
            Self::Anonymous => String::from("ANONYMOUS"),
        }
    }
}

impl Default for PermissionRoleType {
    fn default() -> PermissionRoleType {
        Self::Predefined
    }
}




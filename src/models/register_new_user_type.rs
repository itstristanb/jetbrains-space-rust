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
pub enum RegisterNewUserType {
    #[serde(rename = "Member")]
    Member,
    #[serde(rename = "ExternalCollaborator")]
    ExternalCollaborator,
    #[serde(rename = "Guest")]
    Guest,

}

impl ToString for RegisterNewUserType {
    fn to_string(&self) -> String {
        match self {
            Self::Member => String::from("Member"),
            Self::ExternalCollaborator => String::from("ExternalCollaborator"),
            Self::Guest => String::from("Guest"),
        }
    }
}

impl Default for RegisterNewUserType {
    fn default() -> RegisterNewUserType {
        Self::Member
    }
}





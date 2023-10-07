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
pub enum Gender {
    #[serde(rename = "Female")]
    Female,
    #[serde(rename = "Male")]
    Male,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "NotSpecified")]
    NotSpecified,

}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Self::Female => String::from("Female"),
            Self::Male => String::from("Male"),
            Self::Other => String::from("Other"),
            Self::NotSpecified => String::from("NotSpecified"),
        }
    }
}

impl Default for Gender {
    fn default() -> Gender {
        Self::Female
    }
}





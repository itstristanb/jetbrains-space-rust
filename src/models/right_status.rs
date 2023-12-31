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
pub enum RightStatus {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "GRANTED")]
    Granted,
    #[serde(rename = "REJECTED")]
    Rejected,

}

impl ToString for RightStatus {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("NONE"),
            Self::Requested => String::from("REQUESTED"),
            Self::Granted => String::from("GRANTED"),
            Self::Rejected => String::from("REJECTED"),
        }
    }
}

impl Default for RightStatus {
    fn default() -> RightStatus {
        Self::None
    }
}





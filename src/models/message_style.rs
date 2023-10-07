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
pub enum MessageStyle {
    #[serde(rename = "PRIMARY")]
    Primary,
    #[serde(rename = "SECONDARY")]
    Secondary,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARNING")]
    Warning,

}

impl ToString for MessageStyle {
    fn to_string(&self) -> String {
        match self {
            Self::Primary => String::from("PRIMARY"),
            Self::Secondary => String::from("SECONDARY"),
            Self::Success => String::from("SUCCESS"),
            Self::Error => String::from("ERROR"),
            Self::Warning => String::from("WARNING"),
        }
    }
}

impl Default for MessageStyle {
    fn default() -> MessageStyle {
        Self::Primary
    }
}




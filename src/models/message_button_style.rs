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
pub enum MessageButtonStyle {
    #[serde(rename = "PRIMARY")]
    Primary,
    #[serde(rename = "REGULAR")]
    Regular,
    #[serde(rename = "DANGER")]
    Danger,
    #[serde(rename = "SECONDARY")]
    Secondary,

}

impl ToString for MessageButtonStyle {
    fn to_string(&self) -> String {
        match self {
            Self::Primary => String::from("PRIMARY"),
            Self::Regular => String::from("REGULAR"),
            Self::Danger => String::from("DANGER"),
            Self::Secondary => String::from("SECONDARY"),
        }
    }
}

impl Default for MessageButtonStyle {
    fn default() -> MessageButtonStyle {
        Self::Primary
    }
}




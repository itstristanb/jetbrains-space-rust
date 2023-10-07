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
pub enum MessageTextSize {
    #[serde(rename = "LARGE")]
    Large,
    #[serde(rename = "REGULAR")]
    Regular,
    #[serde(rename = "SMALL")]
    Small,

}

impl ToString for MessageTextSize {
    fn to_string(&self) -> String {
        match self {
            Self::Large => String::from("LARGE"),
            Self::Regular => String::from("REGULAR"),
            Self::Small => String::from("SMALL"),
        }
    }
}

impl Default for MessageTextSize {
    fn default() -> MessageTextSize {
        Self::Large
    }
}




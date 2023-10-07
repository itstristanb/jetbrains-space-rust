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
pub enum MessageTimestampFormat {
    #[serde(rename = "RELATIVE_TO_NOW")]
    RelativeToNow,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "TIME_ONLY")]
    TimeOnly,
    #[serde(rename = "DATE_ONLY")]
    DateOnly,

}

impl ToString for MessageTimestampFormat {
    fn to_string(&self) -> String {
        match self {
            Self::RelativeToNow => String::from("RELATIVE_TO_NOW"),
            Self::Full => String::from("FULL"),
            Self::TimeOnly => String::from("TIME_ONLY"),
            Self::DateOnly => String::from("DATE_ONLY"),
        }
    }
}

impl Default for MessageTimestampFormat {
    fn default() -> MessageTimestampFormat {
        Self::RelativeToNow
    }
}





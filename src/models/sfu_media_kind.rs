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
pub enum SfuMediaKind {
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "audio")]
    Audio,

}

impl ToString for SfuMediaKind {
    fn to_string(&self) -> String {
        match self {
            Self::Video => String::from("video"),
            Self::Audio => String::from("audio"),
        }
    }
}

impl Default for SfuMediaKind {
    fn default() -> SfuMediaKind {
        Self::Video
    }
}





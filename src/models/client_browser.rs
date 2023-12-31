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
pub enum ClientBrowser {
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Chrome")]
    Chrome,
    #[serde(rename = "Firefox")]
    Firefox,
    #[serde(rename = "Safari")]
    Safari,
    #[serde(rename = "Edge")]
    Edge,
    #[serde(rename = "Opera")]
    Opera,

}

impl ToString for ClientBrowser {
    fn to_string(&self) -> String {
        match self {
            Self::Other => String::from("Other"),
            Self::Chrome => String::from("Chrome"),
            Self::Firefox => String::from("Firefox"),
            Self::Safari => String::from("Safari"),
            Self::Edge => String::from("Edge"),
            Self::Opera => String::from("Opera"),
        }
    }
}

impl Default for ClientBrowser {
    fn default() -> ClientBrowser {
        Self::Other
    }
}





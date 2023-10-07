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
pub enum UnfurlViewType {
    #[serde(rename = "Inline")]
    Inline,
    #[serde(rename = "Attachment")]
    Attachment,

}

impl ToString for UnfurlViewType {
    fn to_string(&self) -> String {
        match self {
            Self::Inline => String::from("Inline"),
            Self::Attachment => String::from("Attachment"),
        }
    }
}

impl Default for UnfurlViewType {
    fn default() -> UnfurlViewType {
        Self::Inline
    }
}





/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RtEmoji {
    #[serde(rename = "emojiName")]
    pub emoji_name: String,
    #[serde(rename = "marks")]
    pub marks: Vec<crate::models::RtDocumentMark>,
}

impl RtEmoji {
    pub fn new(emoji_name: String, marks: Vec<crate::models::RtDocumentMark>) -> RtEmoji {
        RtEmoji {
            emoji_name,
            marks,
        }
    }
}



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
pub struct TextDocumentRecord {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "type")]
    pub r#type: crate::models::DraftDocumentType,
    #[serde(rename = "attachments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<crate::models::AttachmentInfo>>>,
    #[serde(rename = "mentions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Option<Vec<crate::models::ResolvedMentionLink>>>,
}

impl TextDocumentRecord {
    pub fn new(id: String, archived: bool, r#type: crate::models::DraftDocumentType) -> TextDocumentRecord {
        TextDocumentRecord {
            id,
            archived,
            r#type,
            attachments: None,
            mentions: None,
        }
    }
}



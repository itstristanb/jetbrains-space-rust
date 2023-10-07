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
pub struct CustomEmojiAddedEvent {
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "owner")]
    pub owner: Box<crate::models::CPrincipal>,
    #[serde(rename = "attachmentId")]
    pub attachment_id: String,
    #[serde(rename = "uploadedAt")]
    pub uploaded_at: String,
}

impl CustomEmojiAddedEvent {
    pub fn new(emoji: String, owner: crate::models::CPrincipal, attachment_id: String, uploaded_at: String) -> CustomEmojiAddedEvent {
        CustomEmojiAddedEvent {
            emoji,
            owner: Box::new(owner),
            attachment_id,
            uploaded_at,
        }
    }
}



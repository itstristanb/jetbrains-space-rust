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
pub struct ImportMessageJbsCreate {
    #[serde(rename = "messageId")]
    pub message_id: Box<crate::models::ChatMessageIdentifierJbsExternalId>,
    #[serde(rename = "message")]
    pub message: Box<crate::models::ChatMessage>,
    #[serde(rename = "author")]
    pub author: Box<crate::models::PrincipalIn>,
    #[serde(rename = "createdAtUtc")]
    pub created_at_utc: i64,
    #[serde(rename = "editedAtUtc", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edited_at_utc: Option<Option<i64>>,
    #[serde(rename = "attachments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<crate::models::AttachmentIn>>>,
}

impl ImportMessageJbsCreate {
    pub fn new(message_id: crate::models::ChatMessageIdentifierJbsExternalId, message: crate::models::ChatMessage, author: crate::models::PrincipalIn, created_at_utc: i64) -> ImportMessageJbsCreate {
        ImportMessageJbsCreate {
            message_id: Box::new(message_id),
            message: Box::new(message),
            author: Box::new(author),
            created_at_utc,
            edited_at_utc: None,
            attachments: None,
        }
    }
}



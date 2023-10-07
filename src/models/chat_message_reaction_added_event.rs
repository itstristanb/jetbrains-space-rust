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
pub struct ChatMessageReactionAddedEvent {
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "threadId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<Option<String>>,
    #[serde(rename = "actor")]
    pub actor: Box<crate::models::CPrincipal>,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "newCount")]
    pub new_count: i32,
}

impl ChatMessageReactionAddedEvent {
    pub fn new(message_id: String, channel_id: String, actor: crate::models::CPrincipal, emoji: String, new_count: i32) -> ChatMessageReactionAddedEvent {
        ChatMessageReactionAddedEvent {
            message_id,
            channel_id,
            thread_id: None,
            actor: Box::new(actor),
            emoji,
            new_count,
        }
    }
}



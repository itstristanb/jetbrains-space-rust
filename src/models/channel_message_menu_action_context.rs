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
pub struct ChannelMessageMenuActionContext {
    #[serde(rename = "channelIdentifier")]
    pub channel_identifier: Box<crate::models::ChannelIdentifier>,
    #[serde(rename = "messageIdentifier")]
    pub message_identifier: Box<crate::models::ChatMessageIdentifier>,
}

impl ChannelMessageMenuActionContext {
    pub fn new(channel_identifier: crate::models::ChannelIdentifier, message_identifier: crate::models::ChatMessageIdentifier) -> ChannelMessageMenuActionContext {
        ChannelMessageMenuActionContext {
            channel_identifier: Box::new(channel_identifier),
            message_identifier: Box::new(message_identifier),
        }
    }
}


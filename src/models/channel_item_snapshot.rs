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
pub struct ChannelItemSnapshot {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "channelId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Option<String>>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<crate::models::M2ItemContentDetails>>,
    #[serde(rename = "author")]
    pub author: Box<crate::models::CPrincipal>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "attachments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<crate::models::AttachmentInfo>>>,
    #[serde(rename = "mentions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Option<Vec<crate::models::EntityMention>>>,
    #[serde(rename = "projectedItem", skip_serializing_if = "Option::is_none")]
    pub projected_item: Option<Box<crate::models::ChannelItemSnapshot>>,
}

impl ChannelItemSnapshot {
    pub fn new(id: String, text: String, author: crate::models::CPrincipal, created: String, time: i64) -> ChannelItemSnapshot {
        ChannelItemSnapshot {
            id,
            channel_id: None,
            text,
            details: None,
            author: Box::new(author),
            created,
            time,
            attachments: None,
            mentions: None,
            projected_item: None,
        }
    }
}



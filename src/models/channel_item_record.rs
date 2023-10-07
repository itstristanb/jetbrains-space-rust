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
pub struct ChannelItemRecord {
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
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Box<crate::models::AllReactionsToItemRecord>>,
    #[serde(rename = "thread", skip_serializing_if = "Option::is_none")]
    pub thread: Option<Box<crate::models::M2ChannelRecord>>,
    #[serde(rename = "projectedItem", skip_serializing_if = "Option::is_none")]
    pub projected_item: Option<Box<crate::models::ChannelItemRecord>>,
    #[serde(rename = "attachments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<crate::models::AttachmentInfo>>>,
    #[serde(rename = "externalId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<Option<String>>,
    #[serde(rename = "pending", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pending: Option<Option<bool>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "edited", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edited: Option<Option<String>>,
    #[serde(rename = "pinned", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pinned: Option<Option<bool>>,
    #[serde(rename = "suggestedParticipants", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suggested_participants: Option<Option<Vec<crate::models::CPrincipal>>>,
    #[serde(rename = "mentions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Option<Vec<crate::models::EntityMention>>>,
    #[serde(rename = "channelId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Option<String>>,
    #[serde(rename = "importerAppId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub importer_app_id: Option<Option<String>>,
    #[serde(rename = "issues")]
    pub issues: Vec<crate::models::Issue>,
    #[serde(rename = "externalIssues", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_issues: Option<Option<Vec<crate::models::ExternalIssue>>>,
}

impl ChannelItemRecord {
    pub fn new(text: String, author: crate::models::CPrincipal, created: String, time: i64, id: String, archived: bool, issues: Vec<crate::models::Issue>) -> ChannelItemRecord {
        ChannelItemRecord {
            text,
            details: None,
            author: Box::new(author),
            created,
            time,
            reactions: None,
            thread: None,
            projected_item: None,
            attachments: None,
            external_id: None,
            pending: None,
            id,
            archived,
            edited: None,
            pinned: None,
            suggested_participants: None,
            mentions: None,
            channel_id: None,
            importer_app_id: None,
            issues,
            external_issues: None,
        }
    }
}


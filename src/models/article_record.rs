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
pub struct ArticleRecord {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "author")]
    pub author: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "aliases")]
    pub aliases: Vec<crate::models::BgArticleAlias>,
    #[serde(rename = "archivedBy", skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<Box<crate::models::TdMemberProfile>>,
    #[serde(rename = "archivedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<Option<String>>,
    #[serde(rename = "channel")]
    pub channel: Box<crate::models::M2ChannelRecord>,
    #[serde(rename = "channelContent", skip_serializing_if = "Option::is_none")]
    pub channel_content: Option<Box<crate::models::M2ChannelContentRecord>>,
    #[serde(rename = "reactions")]
    pub reactions: Box<crate::models::AllReactionsToItemRecord>,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<crate::models::MeetingRecord>>,
    #[serde(rename = "teams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Option<Vec<crate::models::TdTeam>>>,
    #[serde(rename = "locations", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Option<Vec<crate::models::TdLocation>>>,
    #[serde(rename = "externalEntityInfo", skip_serializing_if = "Option::is_none")]
    pub external_entity_info: Option<Box<crate::models::ExternalEntityInfoRecord>>,
    #[serde(rename = "docContent")]
    pub doc_content: Box<crate::models::MdTextDocumentContent>,
    #[serde(rename = "attachments")]
    pub attachments: Vec<crate::models::AttachmentInfo>,
    #[serde(rename = "previewImages")]
    pub preview_images: Vec<crate::models::ArticleMarkdownImage>,
    #[serde(rename = "preview")]
    pub preview: String,
    #[serde(rename = "previewAttachments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preview_attachments: Option<Option<Vec<crate::models::AttachmentInfo>>>,
    #[serde(rename = "wordsNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub words_number: Option<Option<i32>>,
    #[serde(rename = "cut", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cut: Option<Option<bool>>,
    #[serde(rename = "editable")]
    pub editable: bool,
}

impl ArticleRecord {
    pub fn new(id: String, archived: bool, title: String, created: String, author: crate::models::TdMemberProfile, aliases: Vec<crate::models::BgArticleAlias>, channel: crate::models::M2ChannelRecord, reactions: crate::models::AllReactionsToItemRecord, content: String, doc_content: crate::models::MdTextDocumentContent, attachments: Vec<crate::models::AttachmentInfo>, preview_images: Vec<crate::models::ArticleMarkdownImage>, preview: String, editable: bool) -> ArticleRecord {
        ArticleRecord {
            id,
            archived,
            title,
            created,
            author: Box::new(author),
            aliases,
            archived_by: None,
            archived_at: None,
            channel: Box::new(channel),
            channel_content: None,
            reactions: Box::new(reactions),
            content,
            event: None,
            teams: None,
            locations: None,
            external_entity_info: None,
            doc_content: Box::new(doc_content),
            attachments,
            preview_images,
            preview,
            preview_attachments: None,
            words_number: None,
            cut: None,
            editable,
        }
    }
}



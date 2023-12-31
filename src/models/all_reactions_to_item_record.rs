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
pub struct AllReactionsToItemRecord {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "emojiReactions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub emoji_reactions: Option<Option<Vec<crate::models::EmojiReactionRecord>>>,
}

impl AllReactionsToItemRecord {
    pub fn new(id: String) -> AllReactionsToItemRecord {
        AllReactionsToItemRecord {
            id,
            emoji_reactions: None,
        }
    }
}



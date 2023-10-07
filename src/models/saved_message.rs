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
pub struct SavedMessage {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "channel")]
    pub channel: Box<crate::models::M2ChannelRecord>,
    #[serde(rename = "message")]
    pub message: Box<crate::models::ChannelItemRecord>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "labels")]
    pub labels: Vec<crate::models::SavedMessageLabel>,
}

impl SavedMessage {
    pub fn new(id: String, channel: crate::models::M2ChannelRecord, message: crate::models::ChannelItemRecord, created: String, archived: bool, labels: Vec<crate::models::SavedMessageLabel>) -> SavedMessage {
        SavedMessage {
            id,
            channel: Box::new(channel),
            message: Box::new(message),
            created,
            archived,
            labels,
        }
    }
}



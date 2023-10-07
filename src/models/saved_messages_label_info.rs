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
pub struct SavedMessagesLabelInfo {
    #[serde(rename = "label")]
    pub label: Box<crate::models::SavedMessageLabel>,
    #[serde(rename = "updated")]
    pub updated: i64,
    #[serde(rename = "messagesCount")]
    pub messages_count: i32,
}

impl SavedMessagesLabelInfo {
    pub fn new(label: crate::models::SavedMessageLabel, updated: i64, messages_count: i32) -> SavedMessagesLabelInfo {
        SavedMessagesLabelInfo {
            label: Box::new(label),
            updated,
            messages_count,
        }
    }
}



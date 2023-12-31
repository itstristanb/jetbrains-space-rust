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
pub struct BasicCrDiscussionContactRecord {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "threadMessage")]
    pub thread_message: Box<crate::models::MessageInfo>,
    #[serde(rename = "channelType")]
    pub channel_type: String,
    #[serde(rename = "contactKey")]
    pub contact_key: String,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl BasicCrDiscussionContactRecord {
    pub fn new(id: String, thread_message: crate::models::MessageInfo, channel_type: String, contact_key: String, archived: bool) -> BasicCrDiscussionContactRecord {
        BasicCrDiscussionContactRecord {
            id,
            thread_message: Box::new(thread_message),
            channel_type,
            contact_key,
            archived,
        }
    }
}



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
pub struct BasicCrUnboundDiscussionContactsRecord {
    #[serde(rename = "discussions")]
    pub discussions: Vec<crate::models::BasicCrUnboundDiscussionContactRecord>,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "id")]
    pub id: String,
}

impl BasicCrUnboundDiscussionContactsRecord {
    pub fn new(discussions: Vec<crate::models::BasicCrUnboundDiscussionContactRecord>, archived: bool, id: String) -> BasicCrUnboundDiscussionContactsRecord {
        BasicCrUnboundDiscussionContactsRecord {
            discussions,
            archived,
            id,
        }
    }
}


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
pub struct BasicCrDiscussionContactsRecord {
    #[serde(rename = "discussions")]
    pub discussions: Vec<crate::models::BasicCrDiscussionContactRecord>,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "id")]
    pub id: String,
}

impl BasicCrDiscussionContactsRecord {
    pub fn new(discussions: Vec<crate::models::BasicCrDiscussionContactRecord>, archived: bool, id: String) -> BasicCrDiscussionContactsRecord {
        BasicCrDiscussionContactsRecord {
            discussions,
            archived,
            id,
        }
    }
}



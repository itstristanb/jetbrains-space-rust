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
pub struct M2ChannelContentThread {
    #[serde(rename = "record")]
    pub record: Box<crate::models::ChannelItemRecord>,
    #[serde(rename = "parent")]
    pub parent: Box<crate::models::M2ChannelRecord>,
}

impl M2ChannelContentThread {
    pub fn new(record: crate::models::ChannelItemRecord, parent: crate::models::M2ChannelRecord) -> M2ChannelContentThread {
        M2ChannelContentThread {
            record: Box::new(record),
            parent: Box::new(parent),
        }
    }
}



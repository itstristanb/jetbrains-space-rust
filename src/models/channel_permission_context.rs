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
pub struct ChannelPermissionContext {
    #[serde(rename = "channel")]
    pub channel: Box<crate::models::M2ChannelRecord>,
}

impl ChannelPermissionContext {
    pub fn new(channel: crate::models::M2ChannelRecord) -> ChannelPermissionContext {
        ChannelPermissionContext {
            channel: Box::new(channel),
        }
    }
}



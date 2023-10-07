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
pub struct ChatsChannelsChannelSubscribersUsersPostRequest {
    #[serde(rename = "profiles")]
    pub profiles: Vec<crate::models::ProfileIdentifier>,
}

impl ChatsChannelsChannelSubscribersUsersPostRequest {
    pub fn new(profiles: Vec<crate::models::ProfileIdentifier>) -> ChatsChannelsChannelSubscribersUsersPostRequest {
        ChatsChannelsChannelSubscribersUsersPostRequest {
            profiles,
        }
    }
}



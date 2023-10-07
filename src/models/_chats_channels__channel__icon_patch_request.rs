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
pub struct ChatsChannelsChannelIconPatchRequest {
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
}

impl ChatsChannelsChannelIconPatchRequest {
    pub fn new() -> ChatsChannelsChannelIconPatchRequest {
        ChatsChannelsChannelIconPatchRequest {
            icon: None,
        }
    }
}


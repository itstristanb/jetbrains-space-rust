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
pub struct OpenPersonalFeedSettingsAction {
    #[serde(rename = "tab")]
    pub tab: String,
    #[serde(rename = "feed")]
    pub feed: String,
    #[serde(rename = "feedId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<Option<String>>,
}

impl OpenPersonalFeedSettingsAction {
    pub fn new(tab: String, feed: String) -> OpenPersonalFeedSettingsAction {
        OpenPersonalFeedSettingsAction {
            tab,
            feed,
            feed_id: None,
        }
    }
}



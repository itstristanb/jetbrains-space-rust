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
pub struct BillingFeedChannel {
    #[serde(rename = "notificationDefaults", skip_serializing_if = "Option::is_none")]
    pub notification_defaults: Option<Box<crate::models::ChannelSpecificDefaults>>,
}

impl BillingFeedChannel {
    pub fn new() -> BillingFeedChannel {
        BillingFeedChannel {
            notification_defaults: None,
        }
    }
}



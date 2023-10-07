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
pub struct NotificationsPersonalCustomSubscriptionsPostRequest {
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::ProfileIdentifier>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "feed")]
    pub feed: String,
    #[serde(rename = "subscription")]
    pub subscription: Box<crate::models::CustomGenericSubscriptionIn>,
}

impl NotificationsPersonalCustomSubscriptionsPostRequest {
    pub fn new(profile: crate::models::ProfileIdentifier, name: String, feed: String, subscription: crate::models::CustomGenericSubscriptionIn) -> NotificationsPersonalCustomSubscriptionsPostRequest {
        NotificationsPersonalCustomSubscriptionsPostRequest {
            profile: Box::new(profile),
            name,
            feed,
            subscription: Box::new(subscription),
        }
    }
}



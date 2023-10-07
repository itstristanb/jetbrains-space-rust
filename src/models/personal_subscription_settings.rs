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
pub struct PersonalSubscriptionSettings {
    #[serde(rename = "feed")]
    pub feed: Box<crate::models::PrivateFeed>,
    #[serde(rename = "subjectSettings")]
    pub subject_settings: Vec<crate::models::PersonalSubscriptionSubjectSettings>,
    #[serde(rename = "enabledCodes")]
    pub enabled_codes: Vec<String>,
}

impl PersonalSubscriptionSettings {
    pub fn new(feed: crate::models::PrivateFeed, subject_settings: Vec<crate::models::PersonalSubscriptionSubjectSettings>, enabled_codes: Vec<String>) -> PersonalSubscriptionSettings {
        PersonalSubscriptionSettings {
            feed: Box::new(feed),
            subject_settings,
            enabled_codes,
        }
    }
}


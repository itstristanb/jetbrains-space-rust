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
pub struct PersonalSubscriptionTarget {
    #[serde(rename = "subjectCode")]
    pub subject_code: String,
    #[serde(rename = "targetCode")]
    pub target_code: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "events")]
    pub events: Vec<crate::models::PersonalSubscriptionEvent>,
    #[serde(rename = "featureFlag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<Option<String>>,
}

impl PersonalSubscriptionTarget {
    pub fn new(subject_code: String, target_code: String, description: String, events: Vec<crate::models::PersonalSubscriptionEvent>) -> PersonalSubscriptionTarget {
        PersonalSubscriptionTarget {
            subject_code,
            target_code,
            description,
            events,
            feature_flag: None,
        }
    }
}



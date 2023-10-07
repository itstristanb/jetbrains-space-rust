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
pub struct TimeTrackingItemsPostRequest {
    #[serde(rename = "subject")]
    pub subject: Box<crate::models::TimeTrackingSubjectIdentifierJbsIssue>,
    #[serde(rename = "userId")]
    pub user_id: Box<crate::models::ProfileIdentifier>,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "duration")]
    pub duration: i32,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl TimeTrackingItemsPostRequest {
    pub fn new(subject: crate::models::TimeTrackingSubjectIdentifierJbsIssue, user_id: crate::models::ProfileIdentifier, date: String, duration: i32) -> TimeTrackingItemsPostRequest {
        TimeTrackingItemsPostRequest {
            subject: Box::new(subject),
            user_id: Box::new(user_id),
            date,
            duration,
            description: None,
        }
    }
}


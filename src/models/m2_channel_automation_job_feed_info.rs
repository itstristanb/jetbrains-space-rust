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
pub struct M2ChannelAutomationJobFeedInfo {
    #[serde(rename = "jobSubscription")]
    pub job_subscription: Box<crate::models::JobSubscription>,
    #[serde(rename = "jobName")]
    pub job_name: String,
    #[serde(rename = "repoName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<Option<String>>,
    #[serde(rename = "notificationDefaults")]
    pub notification_defaults: Box<crate::models::ChannelSpecificDefaults>,
}

impl M2ChannelAutomationJobFeedInfo {
    pub fn new(job_subscription: crate::models::JobSubscription, job_name: String, notification_defaults: crate::models::ChannelSpecificDefaults) -> M2ChannelAutomationJobFeedInfo {
        M2ChannelAutomationJobFeedInfo {
            job_subscription: Box::new(job_subscription),
            job_name,
            repo_name: None,
            notification_defaults: Box::new(notification_defaults),
        }
    }
}



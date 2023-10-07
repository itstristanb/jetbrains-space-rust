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
pub struct M2ChannelContentCodeReviewDiscussion {
    #[serde(rename = "codeReviewDiscussion")]
    pub code_review_discussion: String,
    #[serde(rename = "notificationDefaults")]
    pub notification_defaults: Box<crate::models::ChannelSpecificDefaults>,
}

impl M2ChannelContentCodeReviewDiscussion {
    pub fn new(code_review_discussion: String, notification_defaults: crate::models::ChannelSpecificDefaults) -> M2ChannelContentCodeReviewDiscussion {
        M2ChannelContentCodeReviewDiscussion {
            code_review_discussion,
            notification_defaults: Box::new(notification_defaults),
        }
    }
}



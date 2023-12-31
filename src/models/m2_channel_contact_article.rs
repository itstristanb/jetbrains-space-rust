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
pub struct M2ChannelContactArticle {
    #[serde(rename = "article")]
    pub article: Box<crate::models::ArticleRecord>,
    #[serde(rename = "notificationDefaults")]
    pub notification_defaults: Box<crate::models::ChannelSpecificDefaults>,
}

impl M2ChannelContactArticle {
    pub fn new(article: crate::models::ArticleRecord, notification_defaults: crate::models::ChannelSpecificDefaults) -> M2ChannelContactArticle {
        M2ChannelContactArticle {
            article: Box::new(article),
            notification_defaults: Box::new(notification_defaults),
        }
    }
}



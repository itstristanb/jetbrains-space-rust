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
pub struct M2ChannelContentArticle {
    #[serde(rename = "article")]
    pub article: Box<crate::models::ArticleRecord>,
    #[serde(rename = "articleContent")]
    pub article_content: Box<crate::models::ArticleContentRecord>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<crate::models::ArticleDetailsRecord>>,
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Box<crate::models::ArticleChannelRecord>>,
}

impl M2ChannelContentArticle {
    pub fn new(article: crate::models::ArticleRecord, article_content: crate::models::ArticleContentRecord) -> M2ChannelContentArticle {
        M2ChannelContentArticle {
            article: Box::new(article),
            article_content: Box::new(article_content),
            details: None,
            channel: None,
        }
    }
}



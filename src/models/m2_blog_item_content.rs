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
pub struct M2BlogItemContent {
    #[serde(rename = "article")]
    pub article: Box<crate::models::ArticleRecord>,
    #[serde(rename = "articleContent")]
    pub article_content: Box<crate::models::ArticleContentRecord>,
    #[serde(rename = "articleDetails")]
    pub article_details: Box<crate::models::ArticleDetailsRecord>,
    #[serde(rename = "articleChannel")]
    pub article_channel: Box<crate::models::ArticleChannelRecord>,
}

impl M2BlogItemContent {
    pub fn new(article: crate::models::ArticleRecord, article_content: crate::models::ArticleContentRecord, article_details: crate::models::ArticleDetailsRecord, article_channel: crate::models::ArticleChannelRecord) -> M2BlogItemContent {
        M2BlogItemContent {
            article: Box::new(article),
            article_content: Box::new(article_content),
            article_details: Box::new(article_details),
            article_channel: Box::new(article_channel),
        }
    }
}


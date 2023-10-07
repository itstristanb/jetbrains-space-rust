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
pub struct M2BlogItemPreview {
    #[serde(rename = "article")]
    pub article: Box<crate::models::ArticleRecord>,
    #[serde(rename = "articlePreview")]
    pub article_preview: Box<crate::models::ArticlePreviewRecord>,
    #[serde(rename = "articleDetails")]
    pub article_details: Box<crate::models::ArticleDetailsRecord>,
    #[serde(rename = "articleChannel")]
    pub article_channel: Box<crate::models::ArticleChannelRecord>,
}

impl M2BlogItemPreview {
    pub fn new(article: crate::models::ArticleRecord, article_preview: crate::models::ArticlePreviewRecord, article_details: crate::models::ArticleDetailsRecord, article_channel: crate::models::ArticleChannelRecord) -> M2BlogItemPreview {
        M2BlogItemPreview {
            article: Box::new(article),
            article_preview: Box::new(article_preview),
            article_details: Box::new(article_details),
            article_channel: Box::new(article_channel),
        }
    }
}


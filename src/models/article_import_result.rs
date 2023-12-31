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
pub struct ArticleImportResult {
    #[serde(rename = "externalId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<Option<String>>,
    #[serde(rename = "article", skip_serializing_if = "Option::is_none")]
    pub article: Option<Box<crate::models::ArticleRecord>>,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<String>>,
}

impl ArticleImportResult {
    pub fn new() -> ArticleImportResult {
        ArticleImportResult {
            external_id: None,
            article: None,
            error: None,
        }
    }
}



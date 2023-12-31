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
pub struct DocumentWebhookEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "document")]
    pub document: String,
    #[serde(rename = "documentRef", skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<Box<crate::models::Document>>,
    #[serde(rename = "changeAuthors")]
    pub change_authors: Vec<crate::models::CPrincipal>,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "base")]
    pub base: String,
}

impl DocumentWebhookEvent {
    pub fn new(meta: crate::models::KMetaMod, document: String, change_authors: Vec<crate::models::CPrincipal>, version: String, base: String) -> DocumentWebhookEvent {
        DocumentWebhookEvent {
            meta: Box::new(meta),
            document,
            document_ref: None,
            change_authors,
            version,
            base,
        }
    }
}



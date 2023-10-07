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
pub struct PackagesPublishing {
    #[serde(rename = "publishingId")]
    pub publishing_id: String,
    #[serde(rename = "source")]
    pub source: Box<crate::models::PublishingSourceJbsPackages>,
    #[serde(rename = "created")]
    pub created: i64,
    #[serde(rename = "started", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub started: Option<Option<i64>>,
    #[serde(rename = "completed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completed: Option<Option<i64>>,
    #[serde(rename = "successful")]
    pub successful: bool,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<String>>,
    #[serde(rename = "principal", skip_serializing_if = "Option::is_none")]
    pub principal: Option<Box<crate::models::CPrincipal>>,
}

impl PackagesPublishing {
    pub fn new(publishing_id: String, source: crate::models::PublishingSourceJbsPackages, created: i64, successful: bool) -> PackagesPublishing {
        PackagesPublishing {
            publishing_id,
            source: Box::new(source),
            created,
            started: None,
            completed: None,
            successful,
            error: None,
            principal: None,
        }
    }
}


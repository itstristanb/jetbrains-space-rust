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
pub struct CRight {
    #[serde(rename = "typeCode")]
    pub type_code: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "featureFlag", skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<Box<crate::models::FeatureFlagInfo>>,
}

impl CRight {
    pub fn new(type_code: String, code: String, title: String) -> CRight {
        CRight {
            type_code,
            code,
            title,
            description: None,
            feature_flag: None,
        }
    }
}


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
pub struct TdProfileLanguage {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<crate::models::TdProfileName>>,
    #[serde(rename = "language")]
    pub language: Box<crate::models::TdLanguage>,
    #[serde(rename = "languageCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Option<String>>,
}

impl TdProfileLanguage {
    pub fn new(language: crate::models::TdLanguage) -> TdProfileLanguage {
        TdProfileLanguage {
            name: None,
            language: Box::new(language),
            language_code: None,
        }
    }
}



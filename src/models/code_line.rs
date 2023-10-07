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
pub struct CodeLine {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "index", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub index: Option<Option<i32>>,
    #[serde(rename = "offset")]
    pub offset: i32,
    #[serde(rename = "syntax", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub syntax: Option<Option<Vec<crate::models::SyntaxMarkup>>>,
}

impl CodeLine {
    pub fn new(text: String, offset: i32) -> CodeLine {
        CodeLine {
            text,
            index: None,
            offset,
            syntax: None,
        }
    }
}


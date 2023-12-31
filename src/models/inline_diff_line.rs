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
pub struct InlineDiffLine {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::DiffLineType>,
    #[serde(rename = "oldLineNum", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub old_line_num: Option<Option<i32>>,
    #[serde(rename = "newLineNum", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new_line_num: Option<Option<i32>>,
    #[serde(rename = "oldFileOffset")]
    pub old_file_offset: i32,
    #[serde(rename = "newFileOffset")]
    pub new_file_offset: i32,
    #[serde(rename = "syntax", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub syntax: Option<Option<Vec<crate::models::SyntaxMarkup>>>,
    #[serde(rename = "deletes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deletes: Option<Option<Vec<crate::models::TextRange>>>,
    #[serde(rename = "inserts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub inserts: Option<Option<Vec<crate::models::TextRange>>>,
}

impl InlineDiffLine {
    pub fn new(text: String, old_file_offset: i32, new_file_offset: i32) -> InlineDiffLine {
        InlineDiffLine {
            text,
            r#type: None,
            old_line_num: None,
            new_line_num: None,
            old_file_offset,
            new_file_offset,
            syntax: None,
            deletes: None,
            inserts: None,
        }
    }
}



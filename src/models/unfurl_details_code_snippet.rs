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
pub struct UnfurlDetailsCodeSnippet {
    #[serde(rename = "anchor")]
    pub anchor: Box<crate::models::CodeSnippetAnchor>,
    #[serde(rename = "lines")]
    pub lines: Vec<crate::models::CodeLine>,
}

impl UnfurlDetailsCodeSnippet {
    pub fn new(anchor: crate::models::CodeSnippetAnchor, lines: Vec<crate::models::CodeLine>) -> UnfurlDetailsCodeSnippet {
        UnfurlDetailsCodeSnippet {
            anchor: Box::new(anchor),
            lines,
        }
    }
}



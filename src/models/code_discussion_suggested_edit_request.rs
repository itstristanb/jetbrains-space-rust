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
pub struct CodeDiscussionSuggestedEditRequest {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "commitId")]
    pub commit_id: String,
    #[serde(rename = "startLineIndex")]
    pub start_line_index: i32,
    #[serde(rename = "endLineIndexInclusive")]
    pub end_line_index_inclusive: i32,
    #[serde(rename = "suggestedContent")]
    pub suggested_content: String,
}

impl CodeDiscussionSuggestedEditRequest {
    pub fn new(path: String, commit_id: String, start_line_index: i32, end_line_index_inclusive: i32, suggested_content: String) -> CodeDiscussionSuggestedEditRequest {
        CodeDiscussionSuggestedEditRequest {
            path,
            commit_id,
            start_line_index,
            end_line_index_inclusive,
            suggested_content,
        }
    }
}



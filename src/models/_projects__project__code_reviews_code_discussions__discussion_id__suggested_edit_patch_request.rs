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
pub struct ProjectsProjectCodeReviewsCodeDiscussionsDiscussionIdSuggestedEditPatchRequest {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "attachments")]
    pub attachments: Vec<crate::models::AttachmentIn>,
    #[serde(rename = "snippetContent", skip_serializing_if = "Option::is_none")]
    pub snippet_content: Option<String>,
}

impl ProjectsProjectCodeReviewsCodeDiscussionsDiscussionIdSuggestedEditPatchRequest {
    pub fn new(text: String, attachments: Vec<crate::models::AttachmentIn>) -> ProjectsProjectCodeReviewsCodeDiscussionsDiscussionIdSuggestedEditPatchRequest {
        ProjectsProjectCodeReviewsCodeDiscussionsDiscussionIdSuggestedEditPatchRequest {
            text,
            attachments,
            snippet_content: None,
        }
    }
}



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
pub struct ProjectsProjectCodeReviewsCodeDiscussionsPostRequest {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "attachments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<crate::models::AttachmentIn>>>,
    #[serde(rename = "diffContext", skip_serializing_if = "Option::is_none")]
    pub diff_context: Option<Box<crate::models::DiffContext>>,
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "revision", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub revision: Option<Option<String>>,
    #[serde(rename = "filename", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filename: Option<Option<String>>,
    #[serde(rename = "line", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub line: Option<Option<i32>>,
    #[serde(rename = "oldLine", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub old_line: Option<Option<i32>>,
    #[serde(rename = "anchor", skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Box<crate::models::LocalCodeDiscussionAnchorIn>>,
    #[serde(rename = "endAnchor", skip_serializing_if = "Option::is_none")]
    pub end_anchor: Option<Box<crate::models::LocalCodeDiscussionAnchorIn>>,
    #[serde(rename = "pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    #[serde(rename = "reviewId")]
    pub review_id: Box<crate::models::ReviewIdentifier>,
    #[serde(rename = "suggestedEdit", skip_serializing_if = "Option::is_none")]
    pub suggested_edit: Option<Box<crate::models::CodeDiscussionSuggestedEditRequest>>,
}

impl ProjectsProjectCodeReviewsCodeDiscussionsPostRequest {
    pub fn new(text: String, repository: String, review_id: crate::models::ReviewIdentifier) -> ProjectsProjectCodeReviewsCodeDiscussionsPostRequest {
        ProjectsProjectCodeReviewsCodeDiscussionsPostRequest {
            text,
            attachments: None,
            diff_context: None,
            repository,
            revision: None,
            filename: None,
            line: None,
            old_line: None,
            anchor: None,
            end_anchor: None,
            pending: None,
            review_id: Box::new(review_id),
            suggested_edit: None,
        }
    }
}



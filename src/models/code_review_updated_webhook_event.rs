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
pub struct CodeReviewUpdatedWebhookEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "review", skip_serializing_if = "Option::is_none")]
    pub review: Option<Box<crate::models::CodeReviewRecord>>,
    #[serde(rename = "titleMod", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_mod: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
    #[serde(rename = "descriptionMod", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description_mod: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
    #[serde(rename = "targetBranchMod", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_branch_mod: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
}

impl CodeReviewUpdatedWebhookEvent {
    pub fn new(meta: crate::models::KMetaMod) -> CodeReviewUpdatedWebhookEvent {
        CodeReviewUpdatedWebhookEvent {
            meta: Box::new(meta),
            review: None,
            title_mod: None,
            description_mod: None,
            target_branch_mod: None,
        }
    }
}



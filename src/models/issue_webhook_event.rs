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
pub struct IssueWebhookEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "issue")]
    pub issue: Box<crate::models::Issue>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
    #[serde(rename = "assignee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Option<Box<crate::models::ApplicationWebhookEventOwner>>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<Box<crate::models::IssueWebhookEventStatus>>>,
    #[serde(rename = "dueDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Option<Box<crate::models::AbsenceWebhookEventSince>>>,
    #[serde(rename = "tagDelta", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tag_delta: Option<Option<Box<crate::models::IssueWebhookEventTagDelta>>>,
    #[serde(rename = "checklistDelta", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub checklist_delta: Option<Option<Box<crate::models::IssueWebhookEventChecklistDelta>>>,
    #[serde(rename = "sprintDelta", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sprint_delta: Option<Option<Box<crate::models::IssueWebhookEventSprintDelta>>>,
    #[serde(rename = "customFieldUpdate", skip_serializing_if = "Option::is_none")]
    pub custom_field_update: Option<Box<crate::models::IssueWebhookCustomFieldUpdate>>,
    #[serde(rename = "deleted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Option<Box<crate::models::AbsenceWebhookEventAvailable>>>,
}

impl IssueWebhookEvent {
    pub fn new(meta: crate::models::KMetaMod, issue: crate::models::Issue) -> IssueWebhookEvent {
        IssueWebhookEvent {
            meta: Box::new(meta),
            issue: Box::new(issue),
            title: None,
            description: None,
            assignee: None,
            status: None,
            due_date: None,
            tag_delta: None,
            checklist_delta: None,
            sprint_delta: None,
            custom_field_update: None,
            deleted: None,
        }
    }
}



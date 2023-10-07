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
pub struct AbsenceWebhookEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "absence")]
    pub absence: Box<crate::models::AbsenceRecord>,
    #[serde(rename = "member")]
    pub member: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
    #[serde(rename = "reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<Box<crate::models::AbsenceWebhookEventReason>>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
    #[serde(rename = "since", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub since: Option<Option<Box<crate::models::AbsenceWebhookEventSince>>>,
    #[serde(rename = "till", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub till: Option<Option<Box<crate::models::AbsenceWebhookEventSince>>>,
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<Box<crate::models::AbsenceWebhookEventLocation>>>,
    #[serde(rename = "available", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub available: Option<Option<Box<crate::models::AbsenceWebhookEventAvailable>>>,
}

impl AbsenceWebhookEvent {
    pub fn new(meta: crate::models::KMetaMod, absence: crate::models::AbsenceRecord, member: crate::models::TdMemberProfile) -> AbsenceWebhookEvent {
        AbsenceWebhookEvent {
            meta: Box::new(meta),
            absence: Box::new(absence),
            member: Box::new(member),
            icon: None,
            reason: None,
            description: None,
            since: None,
            till: None,
            location: None,
            available: None,
        }
    }
}


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
pub struct BusinessEntityRelationEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "relation")]
    pub relation: String,
    #[serde(rename = "entity")]
    pub entity: String,
    #[serde(rename = "member")]
    pub member: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "since", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub since: Option<Option<Box<crate::models::AbsenceWebhookEventSince>>>,
    #[serde(rename = "till", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub till: Option<Option<Box<crate::models::AbsenceWebhookEventSince>>>,
    #[serde(rename = "archived", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub archived: Option<Option<Box<crate::models::AbsenceWebhookEventAvailable>>>,
}

impl BusinessEntityRelationEvent {
    pub fn new(meta: crate::models::KMetaMod, relation: String, entity: String, member: crate::models::TdMemberProfile) -> BusinessEntityRelationEvent {
        BusinessEntityRelationEvent {
            meta: Box::new(meta),
            relation,
            entity,
            member: Box::new(member),
            since: None,
            till: None,
            archived: None,
        }
    }
}



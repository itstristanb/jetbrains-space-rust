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
pub struct FeatureFlagWebhookEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "issueNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issue_number: Option<Option<i32>>,
    #[serde(rename = "enabledForAll", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enabled_for_all: Option<Option<Box<crate::models::AbsenceWebhookEventAvailable>>>,
    #[serde(rename = "selfManageable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub self_manageable: Option<Option<Box<crate::models::AbsenceWebhookEventAvailable>>>,
    #[serde(rename = "addedTeams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub added_teams: Option<Option<Vec<crate::models::TdTeam>>>,
    #[serde(rename = "addedProfiles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub added_profiles: Option<Option<Vec<crate::models::TdTeam>>>,
    #[serde(rename = "removedTeams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub removed_teams: Option<Option<Vec<crate::models::TdMemberProfile>>>,
    #[serde(rename = "removedProfiles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub removed_profiles: Option<Option<Vec<crate::models::TdMemberProfile>>>,
}

impl FeatureFlagWebhookEvent {
    pub fn new(meta: crate::models::KMetaMod, name: String) -> FeatureFlagWebhookEvent {
        FeatureFlagWebhookEvent {
            meta: Box::new(meta),
            name,
            issue_number: None,
            enabled_for_all: None,
            self_manageable: None,
            added_teams: None,
            added_profiles: None,
            removed_teams: None,
            removed_profiles: None,
        }
    }
}



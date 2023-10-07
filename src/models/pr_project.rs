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
pub struct PrProject {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "key")]
    pub key: Box<crate::models::ProjectKey>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "private")]
    pub private: bool,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(rename = "latestRepositoryActivity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latest_repository_activity: Option<Option<String>>,
    #[serde(rename = "createdAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "guestProfiles")]
    pub guest_profiles: Vec<crate::models::TdMemberProfile>,
    #[serde(rename = "type")]
    pub r#type: crate::models::ProjectTeamType,
    #[serde(rename = "teams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Option<Vec<crate::models::TdTeam>>>,
    #[serde(rename = "members", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub members: Option<Option<Vec<crate::models::ProjectTeamMemberRecord>>>,
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<Box<crate::models::TdTeam>>,
    #[serde(rename = "features", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub features: Option<Option<Vec<crate::models::ProjectFeatureState>>>,
    #[serde(rename = "featuresUsage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub features_usage: Option<Option<Vec<crate::models::ProjectFeatureUsage>>>,
    #[serde(rename = "boards")]
    pub boards: Vec<crate::models::BoardRecord>,
    #[serde(rename = "repos")]
    pub repos: Vec<crate::models::PrRepositoryInfo>,
    #[serde(rename = "personalFeaturePins", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub personal_feature_pins: Option<Option<Vec<crate::models::ToggleableProjectFeaturePins>>>,
    #[serde(rename = "enable")]
    pub enable: bool,
    #[serde(rename = "hoursInDay")]
    pub hours_in_day: i32,
    #[serde(rename = "daysInWeek")]
    pub days_in_week: i32,
    #[serde(rename = "format")]
    pub format: crate::models::DurationTextFormat,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "packages")]
    pub packages: Vec<crate::models::ProjectPackageRepository>,
    #[serde(rename = "collaboratorsProfiles")]
    pub collaborators_profiles: Vec<crate::models::TdMemberProfile>,
    #[serde(rename = "collaboratorsTeams")]
    pub collaborators_teams: Vec<crate::models::TdTeam>,
    #[serde(rename = "trackers")]
    pub trackers: Vec<crate::models::ProjectIssueTrackerItem>,
    #[serde(rename = "bundles")]
    pub bundles: Vec<crate::models::ProjectParameterBundle>,
    #[serde(rename = "featurePins", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub feature_pins: Option<Option<Vec<crate::models::CommonProjectFeaturePins>>>,
    #[serde(rename = "memberProfiles")]
    pub member_profiles: Vec<crate::models::TdMemberProfile>,
    #[serde(rename = "memberTeams")]
    pub member_teams: Vec<crate::models::TdTeam>,
    #[serde(rename = "adminProfiles")]
    pub admin_profiles: Vec<crate::models::TdMemberProfile>,
    #[serde(rename = "adminTeams")]
    pub admin_teams: Vec<crate::models::TdTeam>,
}

impl PrProject {
    pub fn new(id: String, key: crate::models::ProjectKey, name: String, private: bool, archived: bool, guest_profiles: Vec<crate::models::TdMemberProfile>, r#type: crate::models::ProjectTeamType, boards: Vec<crate::models::BoardRecord>, repos: Vec<crate::models::PrRepositoryInfo>, enable: bool, hours_in_day: i32, days_in_week: i32, format: crate::models::DurationTextFormat, tags: Vec<String>, packages: Vec<crate::models::ProjectPackageRepository>, collaborators_profiles: Vec<crate::models::TdMemberProfile>, collaborators_teams: Vec<crate::models::TdTeam>, trackers: Vec<crate::models::ProjectIssueTrackerItem>, bundles: Vec<crate::models::ProjectParameterBundle>, member_profiles: Vec<crate::models::TdMemberProfile>, member_teams: Vec<crate::models::TdTeam>, admin_profiles: Vec<crate::models::TdMemberProfile>, admin_teams: Vec<crate::models::TdTeam>) -> PrProject {
        PrProject {
            id,
            key: Box::new(key),
            name,
            private,
            description: None,
            icon: None,
            latest_repository_activity: None,
            created_at: None,
            archived,
            guest_profiles,
            r#type,
            teams: None,
            members: None,
            team: None,
            features: None,
            features_usage: None,
            boards,
            repos,
            personal_feature_pins: None,
            enable,
            hours_in_day,
            days_in_week,
            format,
            tags,
            packages,
            collaborators_profiles,
            collaborators_teams,
            trackers,
            bundles,
            feature_pins: None,
            member_profiles,
            member_teams,
            admin_profiles,
            admin_teams,
        }
    }
}


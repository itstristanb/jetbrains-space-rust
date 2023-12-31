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
pub struct TdMemberProfile {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "name")]
    pub name: Box<crate::models::TdProfileName>,
    #[serde(rename = "speaksEnglish")]
    pub speaks_english: bool,
    #[serde(rename = "smallAvatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub small_avatar: Option<Option<String>>,
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    #[serde(rename = "profilePicture", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<Option<String>>,
    #[serde(rename = "languages")]
    pub languages: Vec<crate::models::TdProfileLanguage>,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "notAMember")]
    pub not_a_member: bool,
    #[serde(rename = "suspended", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<Option<bool>>,
    #[serde(rename = "suspendedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<Option<String>>,
    #[serde(rename = "joined", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub joined: Option<Option<String>>,
    #[serde(rename = "leftAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub left_at: Option<Option<String>>,
    #[serde(rename = "external", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external: Option<Option<bool>>,
    #[serde(rename = "externalLight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_light: Option<Option<bool>>,
    #[serde(rename = "managers")]
    pub managers: Vec<crate::models::TdMemberProfile>,
    #[serde(rename = "onboardingRequired")]
    pub onboarding_required: bool,
    #[serde(rename = "showBannerOnLandingPage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub show_banner_on_landing_page: Option<Option<bool>>,
    #[serde(rename = "showBannerOnProjectPage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub show_banner_on_project_page: Option<Option<bool>>,
    #[serde(rename = "showBannerOnTeamDirectoryHomePage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub show_banner_on_team_directory_home_page: Option<Option<bool>>,
    #[serde(rename = "accessSuspended", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_suspended: Option<Option<bool>>,
    #[serde(rename = "accessSuspendedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_suspended_at: Option<Option<String>>,
    #[serde(rename = "left", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub left: Option<Option<String>>,
    #[serde(rename = "about", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub about: Option<Option<String>>,
    #[serde(rename = "avatarCropSquare", skip_serializing_if = "Option::is_none")]
    pub avatar_crop_square: Option<Box<crate::models::AvatarCropSquare>>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<crate::models::Gender>,
    #[serde(rename = "birthday", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub birthday: Option<Option<String>>,
    #[serde(rename = "memberships")]
    pub memberships: Vec<crate::models::TdMembership>,
    #[serde(rename = "unapprovedMemberships", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unapproved_memberships: Option<Option<Vec<crate::models::TdMembership>>>,
    #[serde(rename = "logins")]
    pub logins: Vec<crate::models::EsProfileLogin>,
    #[serde(rename = "membershipHistory")]
    pub membership_history: Vec<crate::models::TdMembership>,
    #[serde(rename = "status")]
    pub status: crate::models::TwoFactorAuthenticationStatus,
    #[serde(rename = "externalId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<Option<String>>,
    #[serde(rename = "locations")]
    pub locations: Vec<crate::models::TdMemberLocation>,
    #[serde(rename = "absences")]
    pub absences: Vec<crate::models::AbsenceRecord>,
    #[serde(rename = "topics")]
    pub topics: Vec<crate::models::Topic>,
    #[serde(rename = "emails")]
    pub emails: Vec<crate::models::TdProfileEmail>,
    #[serde(rename = "phones")]
    pub phones: Vec<String>,
    #[serde(rename = "messengers")]
    pub messengers: Vec<String>,
    #[serde(rename = "links")]
    pub links: Vec<String>,
    #[serde(rename = "folderWithChildren")]
    pub folder_with_children: Box<crate::models::DocumentFolderWithChildren>,
    #[serde(rename = "locationHistory")]
    pub location_history: Vec<crate::models::TdMemberLocation>,
    #[serde(rename = "customFields")]
    pub custom_fields: ::std::collections::HashMap<String, crate::models::CfValue>,
}

impl TdMemberProfile {
    pub fn new(id: String, username: String, name: crate::models::TdProfileName, speaks_english: bool, languages: Vec<crate::models::TdProfileLanguage>, archived: bool, not_a_member: bool, managers: Vec<crate::models::TdMemberProfile>, onboarding_required: bool, memberships: Vec<crate::models::TdMembership>, logins: Vec<crate::models::EsProfileLogin>, membership_history: Vec<crate::models::TdMembership>, status: crate::models::TwoFactorAuthenticationStatus, locations: Vec<crate::models::TdMemberLocation>, absences: Vec<crate::models::AbsenceRecord>, topics: Vec<crate::models::Topic>, emails: Vec<crate::models::TdProfileEmail>, phones: Vec<String>, messengers: Vec<String>, links: Vec<String>, folder_with_children: crate::models::DocumentFolderWithChildren, location_history: Vec<crate::models::TdMemberLocation>, custom_fields: ::std::collections::HashMap<String, crate::models::CfValue>) -> TdMemberProfile {
        TdMemberProfile {
            id,
            username,
            name: Box::new(name),
            speaks_english,
            small_avatar: None,
            avatar: None,
            profile_picture: None,
            languages,
            archived,
            not_a_member,
            suspended: None,
            suspended_at: None,
            joined: None,
            left_at: None,
            external: None,
            external_light: None,
            managers,
            onboarding_required,
            show_banner_on_landing_page: None,
            show_banner_on_project_page: None,
            show_banner_on_team_directory_home_page: None,
            access_suspended: None,
            access_suspended_at: None,
            left: None,
            about: None,
            avatar_crop_square: None,
            gender: None,
            birthday: None,
            memberships,
            unapproved_memberships: None,
            logins,
            membership_history,
            status,
            external_id: None,
            locations,
            absences,
            topics,
            emails,
            phones,
            messengers,
            links,
            folder_with_children: Box::new(folder_with_children),
            location_history,
            custom_fields,
        }
    }
}



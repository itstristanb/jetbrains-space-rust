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
pub struct FollowedColleagueSettingsDto {
    #[serde(rename = "followedMembers")]
    pub followed_members: Box<crate::models::FollowedMembersSettings>,
    #[serde(rename = "projectAndTeams")]
    pub project_and_teams: Vec<crate::models::FollowedEntityDto>,
}

impl FollowedColleagueSettingsDto {
    pub fn new(followed_members: crate::models::FollowedMembersSettings, project_and_teams: Vec<crate::models::FollowedEntityDto>) -> FollowedColleagueSettingsDto {
        FollowedColleagueSettingsDto {
            followed_members: Box::new(followed_members),
            project_and_teams,
        }
    }
}



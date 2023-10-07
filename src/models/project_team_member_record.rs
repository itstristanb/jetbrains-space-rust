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
pub struct ProjectTeamMemberRecord {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "project")]
    pub project: Box<crate::models::PrProject>,
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::TdRole>>,
    #[serde(rename = "since")]
    pub since: String,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl ProjectTeamMemberRecord {
    pub fn new(id: String, project: crate::models::PrProject, profile: crate::models::TdMemberProfile, since: String, archived: bool) -> ProjectTeamMemberRecord {
        ProjectTeamMemberRecord {
            id,
            project: Box::new(project),
            profile: Box::new(profile),
            position: None,
            since,
            archived,
        }
    }
}


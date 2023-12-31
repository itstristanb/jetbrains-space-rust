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
pub struct TeamDirectoryInvitationLinksPostRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    #[serde(rename = "inviteeLimit")]
    pub invitee_limit: i32,
    #[serde(rename = "team", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub team: Option<Option<String>>,
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<String>>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<crate::models::ProjectIdentifier>>,
    #[serde(rename = "projectRole", skip_serializing_if = "Option::is_none")]
    pub project_role: Option<Box<crate::models::ProjectTeamRole>>,
    #[serde(rename = "globalRole", skip_serializing_if = "Option::is_none")]
    pub global_role: Option<Box<crate::models::GlobalRole>>,
}

impl TeamDirectoryInvitationLinksPostRequest {
    pub fn new(name: String, expires_at: String, invitee_limit: i32) -> TeamDirectoryInvitationLinksPostRequest {
        TeamDirectoryInvitationLinksPostRequest {
            name,
            expires_at,
            invitee_limit,
            team: None,
            role: None,
            project: None,
            project_role: None,
            global_role: None,
        }
    }
}



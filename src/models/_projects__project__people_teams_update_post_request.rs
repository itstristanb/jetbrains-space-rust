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
pub struct ProjectsProjectPeopleTeamsUpdatePostRequest {
    #[serde(rename = "team")]
    pub team: Box<crate::models::TeamIdentifier>,
    #[serde(rename = "addRoles")]
    pub add_roles: Vec<crate::models::ProjectRoleIn>,
    #[serde(rename = "removeRoles")]
    pub remove_roles: Vec<crate::models::ProjectRoleIn>,
}

impl ProjectsProjectPeopleTeamsUpdatePostRequest {
    pub fn new(team: crate::models::TeamIdentifier, add_roles: Vec<crate::models::ProjectRoleIn>, remove_roles: Vec<crate::models::ProjectRoleIn>) -> ProjectsProjectPeopleTeamsUpdatePostRequest {
        ProjectsProjectPeopleTeamsUpdatePostRequest {
            team: Box::new(team),
            add_roles,
            remove_roles,
        }
    }
}


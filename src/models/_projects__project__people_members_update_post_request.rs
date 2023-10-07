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
pub struct ProjectsProjectPeopleMembersUpdatePostRequest {
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::ProfileIdentifier>,
    #[serde(rename = "addRoles")]
    pub add_roles: Vec<crate::models::ProjectRoleIn>,
    #[serde(rename = "removeRoles")]
    pub remove_roles: Vec<crate::models::ProjectRoleIn>,
}

impl ProjectsProjectPeopleMembersUpdatePostRequest {
    pub fn new(profile: crate::models::ProfileIdentifier, add_roles: Vec<crate::models::ProjectRoleIn>, remove_roles: Vec<crate::models::ProjectRoleIn>) -> ProjectsProjectPeopleMembersUpdatePostRequest {
        ProjectsProjectPeopleMembersUpdatePostRequest {
            profile: Box::new(profile),
            add_roles,
            remove_roles,
        }
    }
}



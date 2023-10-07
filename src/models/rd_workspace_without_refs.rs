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
pub struct RdWorkspaceWithoutRefs {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ideType")]
    pub ide_type: Box<crate::models::IdeType>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "state")]
    pub state: crate::models::RdWorkspaceState,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "vcsData", skip_serializing_if = "Option::is_none")]
    pub vcs_data: Option<Box<crate::models::RdWorkspaceVcsData>>,
    #[serde(rename = "resources")]
    pub resources: Box<crate::models::RdWorkspaceResources>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "updated")]
    pub updated: String,
    #[serde(rename = "ideVersion", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ide_version: Option<Option<String>>,
}

impl RdWorkspaceWithoutRefs {
    pub fn new(id: String, ide_type: crate::models::IdeType, name: String, state: crate::models::RdWorkspaceState, project_id: String, number: i32, external_id: String, resources: crate::models::RdWorkspaceResources, created: String, updated: String) -> RdWorkspaceWithoutRefs {
        RdWorkspaceWithoutRefs {
            id,
            ide_type: Box::new(ide_type),
            name,
            state,
            project_id,
            number,
            external_id,
            vcs_data: None,
            resources: Box::new(resources),
            created,
            updated,
            ide_version: None,
        }
    }
}



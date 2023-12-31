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
pub struct PlanningTag {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<crate::models::PlanningTag>>,
    #[serde(rename = "name")]
    pub name: String,
}

impl PlanningTag {
    pub fn new(id: String, archived: bool, project_id: String, name: String) -> PlanningTag {
        PlanningTag {
            id,
            archived,
            project_id,
            parent: None,
            name,
        }
    }
}



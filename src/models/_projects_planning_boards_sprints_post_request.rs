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
pub struct ProjectsPlanningBoardsSprintsPostRequest {
    #[serde(rename = "board")]
    pub board: Box<crate::models::BoardIdentifierJbsId>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "launch", skip_serializing_if = "Option::is_none")]
    pub launch: Option<Box<crate::models::SprintLaunch>>,
}

impl ProjectsPlanningBoardsSprintsPostRequest {
    pub fn new(board: crate::models::BoardIdentifierJbsId, name: String, from: String, to: String) -> ProjectsPlanningBoardsSprintsPostRequest {
        ProjectsPlanningBoardsSprintsPostRequest {
            board: Box::new(board),
            name,
            description: None,
            from,
            to,
            launch: None,
        }
    }
}



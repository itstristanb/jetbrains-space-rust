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
pub struct BoardBacklog {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "board")]
    pub board: Box<crate::models::BoardRecord>,
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::BacklogType>,
}

impl BoardBacklog {
    pub fn new(id: String, archived: bool, board: crate::models::BoardRecord, r#type: crate::models::BacklogType) -> BoardBacklog {
        BoardBacklog {
            id,
            archived,
            board: Box::new(board),
            r#type: Box::new(r#type),
        }
    }
}


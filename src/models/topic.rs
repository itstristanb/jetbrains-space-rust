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
pub struct Topic {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<crate::models::Topic>>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<crate::models::PrProject>>,
    #[serde(rename = "responsible")]
    pub responsible: Vec<crate::models::TdMemberProfile>,
}

impl Topic {
    pub fn new(id: String, archived: bool, name: String, responsible: Vec<crate::models::TdMemberProfile>) -> Topic {
        Topic {
            id,
            archived,
            name,
            parent: None,
            project: None,
            responsible,
        }
    }
}



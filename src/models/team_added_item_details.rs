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
pub struct TeamAddedItemDetails {
    #[serde(rename = "team")]
    pub team: Box<crate::models::TdTeam>,
}

impl TeamAddedItemDetails {
    pub fn new(team: crate::models::TdTeam) -> TeamAddedItemDetails {
        TeamAddedItemDetails {
            team: Box::new(team),
        }
    }
}



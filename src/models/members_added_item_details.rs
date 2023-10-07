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
pub struct MembersAddedItemDetails {
    #[serde(rename = "principals")]
    pub principals: Vec<crate::models::CPrincipal>,
    #[serde(rename = "othersDisplayNames")]
    pub others_display_names: Vec<String>,
}

impl MembersAddedItemDetails {
    pub fn new(principals: Vec<crate::models::CPrincipal>, others_display_names: Vec<String>) -> MembersAddedItemDetails {
        MembersAddedItemDetails {
            principals,
            others_display_names,
        }
    }
}



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
pub struct PlanningTagInJbsId {
    #[serde(rename = "id")]
    pub id: String,
}

impl PlanningTagInJbsId {
    pub fn new(id: String) -> PlanningTagInJbsId {
        PlanningTagInJbsId {
            id,
        }
    }
}



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
pub struct EsTeamMapping {
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "externalGroupName")]
    pub external_group_name: String,
}

impl EsTeamMapping {
    pub fn new(team_id: String, external_group_name: String) -> EsTeamMapping {
        EsTeamMapping {
            team_id,
            external_group_name,
        }
    }
}



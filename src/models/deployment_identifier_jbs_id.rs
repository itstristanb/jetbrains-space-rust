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
pub struct DeploymentIdentifierJbsId {
    #[serde(rename = "id")]
    pub id: String,
}

impl DeploymentIdentifierJbsId {
    pub fn new(id: String) -> DeploymentIdentifierJbsId {
        DeploymentIdentifierJbsId {
            id,
        }
    }
}



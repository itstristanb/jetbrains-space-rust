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
pub struct ApplicationIdentifierJbsId {
    #[serde(rename = "id")]
    pub id: String,
}

impl ApplicationIdentifierJbsId {
    pub fn new(id: String) -> ApplicationIdentifierJbsId {
        ApplicationIdentifierJbsId {
            id,
        }
    }
}



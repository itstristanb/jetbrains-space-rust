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
pub struct TeamDirectoryLocationsRestorePostRequest {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

impl TeamDirectoryLocationsRestorePostRequest {
    pub fn new(ids: Vec<String>) -> TeamDirectoryLocationsRestorePostRequest {
        TeamDirectoryLocationsRestorePostRequest {
            ids,
        }
    }
}


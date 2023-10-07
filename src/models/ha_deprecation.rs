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
pub struct HaDeprecation {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "since")]
    pub since: String,
    #[serde(rename = "forRemoval")]
    pub for_removal: bool,
}

impl HaDeprecation {
    pub fn new(message: String, since: String, for_removal: bool) -> HaDeprecation {
        HaDeprecation {
            message,
            since,
            for_removal,
        }
    }
}


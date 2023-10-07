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
pub struct ProfileCfInputValue {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<crate::models::ProfileIdentifier>>,
}

impl ProfileCfInputValue {
    pub fn new() -> ProfileCfInputValue {
        ProfileCfInputValue {
            profile: None,
        }
    }
}



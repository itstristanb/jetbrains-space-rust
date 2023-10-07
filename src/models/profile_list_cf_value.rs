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
pub struct ProfileListCfValue {
    #[serde(rename = "profiles")]
    pub profiles: Vec<crate::models::TdMemberProfile>,
}

impl ProfileListCfValue {
    pub fn new(profiles: Vec<crate::models::TdMemberProfile>) -> ProfileListCfValue {
        ProfileListCfValue {
            profiles,
        }
    }
}



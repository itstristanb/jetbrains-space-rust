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
pub struct ProfileEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "member")]
    pub member: Box<crate::models::TdMemberProfile>,
}

impl ProfileEvent {
    pub fn new(meta: crate::models::KMetaMod, member: crate::models::TdMemberProfile) -> ProfileEvent {
        ProfileEvent {
            meta: Box::new(meta),
            member: Box::new(member),
        }
    }
}



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
pub struct M2MembershipCreatedContent {
    #[serde(rename = "membership")]
    pub membership: Box<crate::models::TdMembership>,
}

impl M2MembershipCreatedContent {
    pub fn new(membership: crate::models::TdMembership) -> M2MembershipCreatedContent {
        M2MembershipCreatedContent {
            membership: Box::new(membership),
        }
    }
}



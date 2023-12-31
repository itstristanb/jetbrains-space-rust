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
pub struct PermissionsRecipientJbsProfile {
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::TdMemberProfile>,
}

impl PermissionsRecipientJbsProfile {
    pub fn new(profile: crate::models::TdMemberProfile) -> PermissionsRecipientJbsProfile {
        PermissionsRecipientJbsProfile {
            profile: Box::new(profile),
        }
    }
}



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
pub struct UnfurlDetailsProfile {
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "strikeThrough", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub strike_through: Option<Option<bool>>,
}

impl UnfurlDetailsProfile {
    pub fn new(profile: crate::models::TdMemberProfile) -> UnfurlDetailsProfile {
        UnfurlDetailsProfile {
            profile: Box::new(profile),
            strike_through: None,
        }
    }
}


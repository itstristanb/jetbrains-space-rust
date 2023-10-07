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
pub struct ChannelIdentifierJbsProfile {
    #[serde(rename = "member")]
    pub member: Box<crate::models::ProfileIdentifier>,
}

impl ChannelIdentifierJbsProfile {
    pub fn new(member: crate::models::ProfileIdentifier) -> ChannelIdentifierJbsProfile {
        ChannelIdentifierJbsProfile {
            member: Box::new(member),
        }
    }
}



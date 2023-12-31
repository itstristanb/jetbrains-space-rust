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
pub struct ChannelIdentifierJbsApplication {
    #[serde(rename = "application")]
    pub application: Box<crate::models::ApplicationIdentifier>,
}

impl ChannelIdentifierJbsApplication {
    pub fn new(application: crate::models::ApplicationIdentifier) -> ChannelIdentifierJbsApplication {
        ChannelIdentifierJbsApplication {
            application: Box::new(application),
        }
    }
}



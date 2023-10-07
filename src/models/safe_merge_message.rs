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
pub struct SafeMergeMessage {
    #[serde(rename = "type")]
    pub r#type: crate::models::SafeMergeMessageJbsType,
    #[serde(rename = "message")]
    pub message: String,
}

impl SafeMergeMessage {
    pub fn new(r#type: crate::models::SafeMergeMessageJbsType, message: String) -> SafeMergeMessage {
        SafeMergeMessage {
            r#type,
            message,
        }
    }
}



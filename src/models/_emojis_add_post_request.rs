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
pub struct EmojisAddPostRequest {
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "attachmentId")]
    pub attachment_id: String,
}

impl EmojisAddPostRequest {
    pub fn new(emoji: String, attachment_id: String) -> EmojisAddPostRequest {
        EmojisAddPostRequest {
            emoji,
            attachment_id,
        }
    }
}



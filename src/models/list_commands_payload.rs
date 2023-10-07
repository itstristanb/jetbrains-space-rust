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
pub struct ListCommandsPayload {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "userId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<String>>,
    #[serde(rename = "verificationToken", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<Option<String>>,
}

impl ListCommandsPayload {
    pub fn new(client_id: String) -> ListCommandsPayload {
        ListCommandsPayload {
            client_id,
            user_id: None,
            verification_token: None,
        }
    }
}



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
pub struct ChangeServerUrlPayload {
    #[serde(rename = "newServerUrl")]
    pub new_server_url: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "userId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<String>>,
    #[serde(rename = "verificationToken", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<Option<String>>,
}

impl ChangeServerUrlPayload {
    pub fn new(new_server_url: String, client_id: String) -> ChangeServerUrlPayload {
        ChangeServerUrlPayload {
            new_server_url,
            client_id,
            user_id: None,
            verification_token: None,
        }
    }
}



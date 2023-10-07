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
pub struct EndpointAuthUpdateDto {
    #[serde(rename = "appLevelAuth", skip_serializing_if = "Option::is_none")]
    pub app_level_auth: Option<crate::models::EndpointAppLevelAuthUpdateType>,
    #[serde(rename = "basicAuthUsername", skip_serializing_if = "Option::is_none")]
    pub basic_auth_username: Option<String>,
    #[serde(rename = "basicAuthPassword", skip_serializing_if = "Option::is_none")]
    pub basic_auth_password: Option<String>,
    #[serde(rename = "bearerAuthToken", skip_serializing_if = "Option::is_none")]
    pub bearer_auth_token: Option<String>,
    #[serde(rename = "hasVerificationToken", skip_serializing_if = "Option::is_none")]
    pub has_verification_token: Option<bool>,
    #[serde(rename = "sslKeystoreAuth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ssl_keystore_auth: Option<Option<String>>,
}

impl EndpointAuthUpdateDto {
    pub fn new() -> EndpointAuthUpdateDto {
        EndpointAuthUpdateDto {
            app_level_auth: None,
            basic_auth_username: None,
            basic_auth_password: None,
            bearer_auth_token: None,
            has_verification_token: None,
            ssl_keystore_auth: None,
        }
    }
}



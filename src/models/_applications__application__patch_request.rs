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
pub struct ApplicationsApplicationPatchRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "pictureAttachmentId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub picture_attachment_id: Option<Option<String>>,
    #[serde(rename = "defaultExternalPicture", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_external_picture: Option<Option<String>>,
    #[serde(rename = "clientSecret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "clientCredentialsFlowEnabled", skip_serializing_if = "Option::is_none")]
    pub client_credentials_flow_enabled: Option<bool>,
    #[serde(rename = "codeFlowEnabled", skip_serializing_if = "Option::is_none")]
    pub code_flow_enabled: Option<bool>,
    #[serde(rename = "codeFlowRedirectURIs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code_flow_redirect_uris: Option<Option<String>>,
    #[serde(rename = "pkceRequired", skip_serializing_if = "Option::is_none")]
    pub pkce_required: Option<bool>,
    #[serde(rename = "publicClientsAllowed", skip_serializing_if = "Option::is_none")]
    pub public_clients_allowed: Option<bool>,
    #[serde(rename = "implicitFlowEnabled", skip_serializing_if = "Option::is_none")]
    pub implicit_flow_enabled: Option<bool>,
    #[serde(rename = "implicitFlowRedirectURIs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implicit_flow_redirect_uris: Option<Option<String>>,
    #[serde(rename = "endpointUri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<Option<String>>,
    #[serde(rename = "endpointSslVerification", skip_serializing_if = "Option::is_none")]
    pub endpoint_ssl_verification: Option<bool>,
    #[serde(rename = "hasVerificationToken", skip_serializing_if = "Option::is_none")]
    pub has_verification_token: Option<bool>,
    #[serde(rename = "hasPublicKeySignature", skip_serializing_if = "Option::is_none")]
    pub has_public_key_signature: Option<bool>,
    #[serde(rename = "hasSigningKey", skip_serializing_if = "Option::is_none")]
    pub has_signing_key: Option<bool>,
    #[serde(rename = "appLevelAuth", skip_serializing_if = "Option::is_none")]
    pub app_level_auth: Option<crate::models::EndpointAppLevelAuthUpdateType>,
    #[serde(rename = "sslKeystoreAuth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ssl_keystore_auth: Option<Option<String>>,
    #[serde(rename = "basicAuthUsername", skip_serializing_if = "Option::is_none")]
    pub basic_auth_username: Option<String>,
    #[serde(rename = "basicAuthPassword", skip_serializing_if = "Option::is_none")]
    pub basic_auth_password: Option<String>,
    #[serde(rename = "bearerAuthToken", skip_serializing_if = "Option::is_none")]
    pub bearer_auth_token: Option<String>,
}

impl ApplicationsApplicationPatchRequest {
    pub fn new() -> ApplicationsApplicationPatchRequest {
        ApplicationsApplicationPatchRequest {
            name: None,
            description: None,
            picture_attachment_id: None,
            default_external_picture: None,
            client_secret: None,
            client_credentials_flow_enabled: None,
            code_flow_enabled: None,
            code_flow_redirect_uris: None,
            pkce_required: None,
            public_clients_allowed: None,
            implicit_flow_enabled: None,
            implicit_flow_redirect_uris: None,
            endpoint_uri: None,
            endpoint_ssl_verification: None,
            has_verification_token: None,
            has_public_key_signature: None,
            has_signing_key: None,
            app_level_auth: None,
            ssl_keystore_auth: None,
            basic_auth_username: None,
            basic_auth_password: None,
            bearer_auth_token: None,
        }
    }
}


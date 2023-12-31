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
pub struct EsApp {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::TdMemberProfile>>,
    #[serde(rename = "ownerApp", skip_serializing_if = "Option::is_none")]
    pub owner_app: Option<Box<crate::models::EsApp>>,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    #[serde(rename = "picture", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub picture: Option<Option<String>>,
    #[serde(rename = "defaultExternalPicture", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_external_picture: Option<Option<String>>,
    #[serde(rename = "createdAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    #[serde(rename = "kind", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Option<String>>,
    #[serde(rename = "presentableName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presentable_name: Option<Option<String>>,
    #[serde(rename = "applicationType", skip_serializing_if = "Option::is_none")]
    pub application_type: Option<crate::models::ApplicationType>,
    #[serde(rename = "clientCredentialsFlowEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_credentials_flow_enabled: Option<Option<bool>>,
    #[serde(rename = "codeFlowEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code_flow_enabled: Option<Option<bool>>,
    #[serde(rename = "codeFlowRedirectURIs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code_flow_redirect_uris: Option<Option<String>>,
    #[serde(rename = "pkceRequired", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pkce_required: Option<Option<bool>>,
    #[serde(rename = "implicitFlowEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implicit_flow_enabled: Option<Option<bool>>,
    #[serde(rename = "implicitFlowRedirectURIs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implicit_flow_redirect_uris: Option<Option<String>>,
    #[serde(rename = "endpointURI", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<Option<String>>,
    #[serde(rename = "hasVerificationToken", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub has_verification_token: Option<Option<bool>>,
    #[serde(rename = "hasSigningKey", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub has_signing_key: Option<Option<bool>>,
    #[serde(rename = "hasPublicKeySignature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub has_public_key_signature: Option<Option<bool>>,
    #[serde(rename = "endpointSslVerification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub endpoint_ssl_verification: Option<Option<bool>>,
    #[serde(rename = "basicAuthUsername", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub basic_auth_username: Option<Option<String>>,
    #[serde(rename = "hasBearerToken", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub has_bearer_token: Option<Option<bool>>,
    #[serde(rename = "sslKeystoreAuth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ssl_keystore_auth: Option<Option<String>>,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::ApplicationMetadata>>,
    #[serde(rename = "settings")]
    pub settings: Box<crate::models::EsAppSettings>,
    #[serde(rename = "errorMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    #[serde(rename = "domains")]
    pub domains: Vec<crate::models::ApplicationUnfurlDomain>,
    #[serde(rename = "patterns")]
    pub patterns: Vec<crate::models::ApplicationUnfurlPattern>,
    #[serde(rename = "contexts")]
    pub contexts: Vec<crate::models::AppUiExtContextData>,
}

impl EsApp {
    pub fn new(id: String, client_id: String, name: String, archived: bool, settings: crate::models::EsAppSettings, domains: Vec<crate::models::ApplicationUnfurlDomain>, patterns: Vec<crate::models::ApplicationUnfurlPattern>, contexts: Vec<crate::models::AppUiExtContextData>) -> EsApp {
        EsApp {
            id,
            owner: None,
            owner_app: None,
            client_id,
            name,
            email: None,
            picture: None,
            default_external_picture: None,
            created_at: None,
            kind: None,
            presentable_name: None,
            application_type: None,
            client_credentials_flow_enabled: None,
            code_flow_enabled: None,
            code_flow_redirect_uris: None,
            pkce_required: None,
            implicit_flow_enabled: None,
            implicit_flow_redirect_uris: None,
            endpoint_uri: None,
            has_verification_token: None,
            has_signing_key: None,
            has_public_key_signature: None,
            endpoint_ssl_verification: None,
            basic_auth_username: None,
            has_bearer_token: None,
            ssl_keystore_auth: None,
            archived,
            description: None,
            metadata: None,
            settings: Box::new(settings),
            error_message: None,
            domains,
            patterns,
            contexts,
        }
    }
}



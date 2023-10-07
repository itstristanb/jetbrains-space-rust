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
pub struct ApplicationWebhookEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "application")]
    pub application: Box<crate::models::EsApp>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
    #[serde(rename = "owner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Option<Box<crate::models::ApplicationWebhookEventOwner>>>,
    #[serde(rename = "archived", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub archived: Option<Option<Box<crate::models::AbsenceWebhookEventAvailable>>>,
    #[serde(rename = "endpointURI", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<Option<Box<crate::models::AbsenceWebhookEventIcon>>>,
    #[serde(rename = "clientIdChanged")]
    pub client_id_changed: bool,
    #[serde(rename = "clientSecretChanged")]
    pub client_secret_changed: bool,
    #[serde(rename = "verificationTokenChanged")]
    pub verification_token_changed: bool,
    #[serde(rename = "signingKeyChanged")]
    pub signing_key_changed: bool,
}

impl ApplicationWebhookEvent {
    pub fn new(meta: crate::models::KMetaMod, application: crate::models::EsApp, client_id_changed: bool, client_secret_changed: bool, verification_token_changed: bool, signing_key_changed: bool) -> ApplicationWebhookEvent {
        ApplicationWebhookEvent {
            meta: Box::new(meta),
            application: Box::new(application),
            name: None,
            owner: None,
            archived: None,
            endpoint_uri: None,
            client_id_changed,
            client_secret_changed,
            verification_token_changed,
            signing_key_changed,
        }
    }
}



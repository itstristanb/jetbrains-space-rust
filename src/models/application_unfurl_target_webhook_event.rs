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
pub struct ApplicationUnfurlTargetWebhookEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "application")]
    pub application: Box<crate::models::EsApp>,
    #[serde(rename = "target")]
    pub target: Box<crate::models::ApplicationUnfurlTarget>,
}

impl ApplicationUnfurlTargetWebhookEvent {
    pub fn new(meta: crate::models::KMetaMod, application: crate::models::EsApp, target: crate::models::ApplicationUnfurlTarget) -> ApplicationUnfurlTargetWebhookEvent {
        ApplicationUnfurlTargetWebhookEvent {
            meta: Box::new(meta),
            application: Box::new(application),
            target: Box::new(target),
        }
    }
}


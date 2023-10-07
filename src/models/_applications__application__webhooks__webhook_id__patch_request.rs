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
pub struct ApplicationsApplicationWebhooksWebhookIdPatchRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Box<crate::models::ExternalEndpointUpdateDto>>,
    #[serde(rename = "endpointAuth", skip_serializing_if = "Option::is_none")]
    pub endpoint_auth: Option<Box<crate::models::EndpointAuthUpdateDto>>,
    #[serde(rename = "acceptedHttpResponseCodes", skip_serializing_if = "Option::is_none")]
    pub accepted_http_response_codes: Option<Vec<i32>>,
    #[serde(rename = "doRetries", skip_serializing_if = "Option::is_none")]
    pub do_retries: Option<bool>,
    #[serde(rename = "payloadFields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payload_fields: Option<Option<String>>,
    #[serde(rename = "payloadTemplate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payload_template: Option<Option<String>>,
}

impl ApplicationsApplicationWebhooksWebhookIdPatchRequest {
    pub fn new() -> ApplicationsApplicationWebhooksWebhookIdPatchRequest {
        ApplicationsApplicationWebhooksWebhookIdPatchRequest {
            name: None,
            description: None,
            enabled: None,
            endpoint: None,
            endpoint_auth: None,
            accepted_http_response_codes: None,
            do_retries: None,
            payload_fields: None,
            payload_template: None,
        }
    }
}


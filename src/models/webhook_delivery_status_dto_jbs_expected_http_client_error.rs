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
pub struct WebhookDeliveryStatusDtoJbsExpectedHttpClientError {
    #[serde(rename = "clientError")]
    pub client_error: Box<crate::models::AppMessageDeliveryClientErrorDto>,
    #[serde(rename = "deliveryId")]
    pub delivery_id: String,
    #[serde(rename = "sentTime")]
    pub sent_time: String,
}

impl WebhookDeliveryStatusDtoJbsExpectedHttpClientError {
    pub fn new(client_error: crate::models::AppMessageDeliveryClientErrorDto, delivery_id: String, sent_time: String) -> WebhookDeliveryStatusDtoJbsExpectedHttpClientError {
        WebhookDeliveryStatusDtoJbsExpectedHttpClientError {
            client_error: Box::new(client_error),
            delivery_id,
            sent_time,
        }
    }
}



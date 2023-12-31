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
pub struct WebhookDeliveryStatusDtoJbsSuccessful {
    #[serde(rename = "deliveryId")]
    pub delivery_id: String,
    #[serde(rename = "responseCode")]
    pub response_code: i32,
    #[serde(rename = "sentTime")]
    pub sent_time: String,
}

impl WebhookDeliveryStatusDtoJbsSuccessful {
    pub fn new(delivery_id: String, response_code: i32, sent_time: String) -> WebhookDeliveryStatusDtoJbsSuccessful {
        WebhookDeliveryStatusDtoJbsSuccessful {
            delivery_id,
            response_code,
            sent_time,
        }
    }
}



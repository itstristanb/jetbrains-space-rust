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
pub struct ProducerOptions {
    #[serde(rename = "sfuOptions")]
    pub sfu_options: Box<crate::models::SfuProducerOptions>,
    #[serde(rename = "userConnectionId")]
    pub user_connection_id: i64,
    #[serde(rename = "routerId")]
    pub router_id: String,
    #[serde(rename = "source")]
    pub source: crate::models::MediaSource,
}

impl ProducerOptions {
    pub fn new(sfu_options: crate::models::SfuProducerOptions, user_connection_id: i64, router_id: String, source: crate::models::MediaSource) -> ProducerOptions {
        ProducerOptions {
            sfu_options: Box::new(sfu_options),
            user_connection_id,
            router_id,
            source,
        }
    }
}


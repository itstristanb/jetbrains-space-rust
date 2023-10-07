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
pub struct MetricsRequest {
    #[serde(rename = "client")]
    pub client: Box<crate::models::ClientInfo>,
    #[serde(rename = "events")]
    pub events: Vec<crate::models::MetricsEvent>,
}

impl MetricsRequest {
    pub fn new(client: crate::models::ClientInfo, events: Vec<crate::models::MetricsEvent>) -> MetricsRequest {
        MetricsRequest {
            client: Box::new(client),
            events,
        }
    }
}


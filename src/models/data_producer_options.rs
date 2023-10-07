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
pub struct DataProducerOptions {
    #[serde(rename = "sfuOptions")]
    pub sfu_options: Box<crate::models::SfuDataProducerOptions>,
    #[serde(rename = "closed")]
    pub closed: bool,
    #[serde(rename = "userConnectionId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_connection_id: Option<Option<i64>>,
    #[serde(rename = "routerId")]
    pub router_id: String,
}

impl DataProducerOptions {
    pub fn new(sfu_options: crate::models::SfuDataProducerOptions, closed: bool, router_id: String) -> DataProducerOptions {
        DataProducerOptions {
            sfu_options: Box::new(sfu_options),
            closed,
            user_connection_id: None,
            router_id,
        }
    }
}



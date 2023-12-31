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
pub struct MetricsEvent {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "props", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub props: Option<Option<Vec<crate::models::MetricsProp>>>,
    #[serde(rename = "points", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub points: Option<Option<Vec<crate::models::MetricsPoint>>>,
}

impl MetricsEvent {
    pub fn new(id: String, time: i64) -> MetricsEvent {
        MetricsEvent {
            id,
            time,
            props: None,
            points: None,
        }
    }
}



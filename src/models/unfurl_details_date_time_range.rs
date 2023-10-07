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
pub struct UnfurlDetailsDateTimeRange {
    #[serde(rename = "since")]
    pub since: i64,
    #[serde(rename = "till")]
    pub till: i64,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<crate::models::DateTimeViewParams>>,
}

impl UnfurlDetailsDateTimeRange {
    pub fn new(since: i64, till: i64) -> UnfurlDetailsDateTimeRange {
        UnfurlDetailsDateTimeRange {
            since,
            till,
            params: None,
        }
    }
}



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
pub struct DateCfFilter {
    #[serde(rename = "minValue", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<Box<crate::models::DateCfValue>>,
    #[serde(rename = "maxValue", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<Box<crate::models::DateCfValue>>,
}

impl DateCfFilter {
    pub fn new() -> DateCfFilter {
        DateCfFilter {
            min_value: None,
            max_value: None,
        }
    }
}



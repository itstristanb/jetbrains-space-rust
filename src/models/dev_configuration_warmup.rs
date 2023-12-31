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
pub struct DevConfigurationWarmup {
    #[serde(rename = "indexing")]
    pub indexing: bool,
    #[serde(rename = "script", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub script: Option<Option<String>>,
    #[serde(rename = "triggers")]
    pub triggers: Vec<crate::models::DevConfigurationWarmupTriggersJbsCron>,
}

impl DevConfigurationWarmup {
    pub fn new(indexing: bool, triggers: Vec<crate::models::DevConfigurationWarmupTriggersJbsCron>) -> DevConfigurationWarmup {
        DevConfigurationWarmup {
            indexing,
            script: None,
            triggers,
        }
    }
}



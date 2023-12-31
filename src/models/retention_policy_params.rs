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
pub struct RetentionPolicyParams {
    #[serde(rename = "numberOfDaysToRetain", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number_of_days_to_retain: Option<Option<i32>>,
    #[serde(rename = "numberOfVersionsToRetain", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number_of_versions_to_retain: Option<Option<i32>>,
    #[serde(rename = "retainDownloadedOnce")]
    pub retain_downloaded_once: bool,
}

impl RetentionPolicyParams {
    pub fn new(retain_downloaded_once: bool) -> RetentionPolicyParams {
        RetentionPolicyParams {
            number_of_days_to_retain: None,
            number_of_versions_to_retain: None,
            retain_downloaded_once,
        }
    }
}



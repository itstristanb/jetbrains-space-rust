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
pub struct WorkerInfoDto {
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<crate::models::WorkerOsType>,
    #[serde(rename = "osName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub os_name: Option<Option<String>>,
    #[serde(rename = "osArch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<Option<String>>,
    #[serde(rename = "osVersion", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<Option<String>>,
    #[serde(rename = "hostname", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Option<String>>,
}

impl WorkerInfoDto {
    pub fn new() -> WorkerInfoDto {
        WorkerInfoDto {
            os_type: None,
            os_name: None,
            os_arch: None,
            os_version: None,
            hostname: None,
        }
    }
}


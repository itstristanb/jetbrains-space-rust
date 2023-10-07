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
pub struct RdInstanceType {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "vcpus")]
    pub vcpus: i32,
    #[serde(rename = "memMb")]
    pub mem_mb: i32,
    #[serde(rename = "volumeGb")]
    pub volume_gb: i32,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "active", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
}

impl RdInstanceType {
    pub fn new(id: String, name: String, vcpus: i32, mem_mb: i32, volume_gb: i32, enabled: bool) -> RdInstanceType {
        RdInstanceType {
            id,
            name,
            vcpus,
            mem_mb,
            volume_gb,
            enabled,
            active: None,
        }
    }
}



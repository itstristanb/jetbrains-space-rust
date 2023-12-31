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
pub struct RdWorkspaceResources {
    #[serde(rename = "milliCpus")]
    pub milli_cpus: i32,
    #[serde(rename = "memoryMb")]
    pub memory_mb: i32,
    #[serde(rename = "volumeGb")]
    pub volume_gb: i32,
}

impl RdWorkspaceResources {
    pub fn new(milli_cpus: i32, memory_mb: i32, volume_gb: i32) -> RdWorkspaceResources {
        RdWorkspaceResources {
            milli_cpus,
            memory_mb,
            volume_gb,
        }
    }
}



/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WorkerOsType {
    #[serde(rename = "Linux")]
    Linux,
    #[serde(rename = "Windows")]
    Windows,
    #[serde(rename = "Mac")]
    Mac,
    #[serde(rename = "Other")]
    Other,

}

impl ToString for WorkerOsType {
    fn to_string(&self) -> String {
        match self {
            Self::Linux => String::from("Linux"),
            Self::Windows => String::from("Windows"),
            Self::Mac => String::from("Mac"),
            Self::Other => String::from("Other"),
        }
    }
}

impl Default for WorkerOsType {
    fn default() -> WorkerOsType {
        Self::Linux
    }
}





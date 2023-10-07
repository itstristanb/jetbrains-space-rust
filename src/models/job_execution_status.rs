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
pub enum JobExecutionStatus {
    #[serde(rename = "Started")]
    Started,
    #[serde(rename = "Succeeded")]
    Succeeded,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Terminated")]
    Terminated,

}

impl ToString for JobExecutionStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Started => String::from("Started"),
            Self::Succeeded => String::from("Succeeded"),
            Self::Failed => String::from("Failed"),
            Self::Terminated => String::from("Terminated"),
        }
    }
}

impl Default for JobExecutionStatus {
    fn default() -> JobExecutionStatus {
        Self::Started
    }
}




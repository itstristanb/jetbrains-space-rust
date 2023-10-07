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
pub enum SafeMergeState {
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "FAILING")]
    Failing,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "CANCELLED")]
    Cancelled,

}

impl ToString for SafeMergeState {
    fn to_string(&self) -> String {
        match self {
            Self::Starting => String::from("STARTING"),
            Self::Running => String::from("RUNNING"),
            Self::Failing => String::from("FAILING"),
            Self::Failed => String::from("FAILED"),
            Self::Succeeded => String::from("SUCCEEDED"),
            Self::Cancelled => String::from("CANCELLED"),
        }
    }
}

impl Default for SafeMergeState {
    fn default() -> SafeMergeState {
        Self::Starting
    }
}




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
pub struct WarmupExecutionTriggerJbsManual {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::TdMemberProfile>>,
}

impl WarmupExecutionTriggerJbsManual {
    pub fn new() -> WarmupExecutionTriggerJbsManual {
        WarmupExecutionTriggerJbsManual {
            user: None,
        }
    }
}



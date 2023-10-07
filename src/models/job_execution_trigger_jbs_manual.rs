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
pub struct JobExecutionTriggerJbsManual {
    #[serde(rename = "principal")]
    pub principal: Box<crate::models::CPrincipal>,
}

impl JobExecutionTriggerJbsManual {
    pub fn new(principal: crate::models::CPrincipal) -> JobExecutionTriggerJbsManual {
        JobExecutionTriggerJbsManual {
            principal: Box::new(principal),
        }
    }
}



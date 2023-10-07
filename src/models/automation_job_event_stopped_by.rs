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
pub struct AutomationJobEventStoppedBy {
    #[serde(rename = "old")]
    pub old: Box<crate::models::CPrincipal>,
    #[serde(rename = "new")]
    pub new: Box<crate::models::CPrincipal>,
}

impl AutomationJobEventStoppedBy {
    pub fn new(old: crate::models::CPrincipal, new: crate::models::CPrincipal) -> AutomationJobEventStoppedBy {
        AutomationJobEventStoppedBy {
            old: Box::new(old),
            new: Box::new(new),
        }
    }
}



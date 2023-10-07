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
pub struct AutomationJobEventStartTime {
    #[serde(rename = "old")]
    pub old: String,
    #[serde(rename = "new")]
    pub new: String,
}

impl AutomationJobEventStartTime {
    pub fn new(old: String, new: String) -> AutomationJobEventStartTime {
        AutomationJobEventStartTime {
            old,
            new,
        }
    }
}



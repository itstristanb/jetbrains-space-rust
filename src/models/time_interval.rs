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
pub struct TimeInterval {
    #[serde(rename = "since")]
    pub since: Box<crate::models::TimeOfDay>,
    #[serde(rename = "till")]
    pub till: Box<crate::models::TimeOfDay>,
}

impl TimeInterval {
    pub fn new(since: crate::models::TimeOfDay, till: crate::models::TimeOfDay) -> TimeInterval {
        TimeInterval {
            since: Box::new(since),
            till: Box::new(till),
        }
    }
}



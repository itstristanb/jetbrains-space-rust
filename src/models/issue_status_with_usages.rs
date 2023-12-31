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
pub struct IssueStatusWithUsages {
    #[serde(rename = "status")]
    pub status: Box<crate::models::IssueStatus>,
    #[serde(rename = "usages")]
    pub usages: i32,
}

impl IssueStatusWithUsages {
    pub fn new(status: crate::models::IssueStatus, usages: i32) -> IssueStatusWithUsages {
        IssueStatusWithUsages {
            status: Box::new(status),
            usages,
        }
    }
}



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
pub struct WorkerScopeFilterJbsProject {
    #[serde(rename = "project")]
    pub project: Box<crate::models::ProjectIdentifier>,
}

impl WorkerScopeFilterJbsProject {
    pub fn new(project: crate::models::ProjectIdentifier) -> WorkerScopeFilterJbsProject {
        WorkerScopeFilterJbsProject {
            project: Box::new(project),
        }
    }
}



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
pub struct ProjectEvent {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::KMetaMod>,
    #[serde(rename = "project")]
    pub project: Box<crate::models::PrProject>,
}

impl ProjectEvent {
    pub fn new(meta: crate::models::KMetaMod, project: crate::models::PrProject) -> ProjectEvent {
        ProjectEvent {
            meta: Box::new(meta),
            project: Box::new(project),
        }
    }
}



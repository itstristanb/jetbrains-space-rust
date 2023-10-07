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
pub struct ProjectScope {
    #[serde(rename = "project")]
    pub project: Box<crate::models::PrProject>,
}

impl ProjectScope {
    pub fn new(project: crate::models::PrProject) -> ProjectScope {
        ProjectScope {
            project: Box::new(project),
        }
    }
}



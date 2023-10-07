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
pub struct DeployTargetsSearchFilterJbsByProjects {
    #[serde(rename = "projects")]
    pub projects: Vec<String>,
    #[serde(rename = "negated")]
    pub negated: bool,
}

impl DeployTargetsSearchFilterJbsByProjects {
    pub fn new(projects: Vec<String>, negated: bool) -> DeployTargetsSearchFilterJbsByProjects {
        DeployTargetsSearchFilterJbsByProjects {
            projects,
            negated,
        }
    }
}



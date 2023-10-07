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
pub struct ProjectsProjectPackagesRepositoriesRepositoryCleanupDryPostRequest {
    #[serde(rename = "retentionParams")]
    pub retention_params: Box<crate::models::RetentionPolicyParams>,
}

impl ProjectsProjectPackagesRepositoriesRepositoryCleanupDryPostRequest {
    pub fn new(retention_params: crate::models::RetentionPolicyParams) -> ProjectsProjectPackagesRepositoriesRepositoryCleanupDryPostRequest {
        ProjectsProjectPackagesRepositoriesRepositoryCleanupDryPostRequest {
            retention_params: Box::new(retention_params),
        }
    }
}


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
pub struct SPackageRepositoryWebhookEvent {
    #[serde(rename = "projectKey")]
    pub project_key: Box<crate::models::ProjectKey>,
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "repositoryType")]
    pub repository_type: Box<crate::models::PackageType>,
    #[serde(rename = "action")]
    pub action: crate::models::PackageRepositoryEventAction,
    #[serde(rename = "packageInfo")]
    pub package_info: Box<crate::models::PackageVersionRef>,
}

impl SPackageRepositoryWebhookEvent {
    pub fn new(project_key: crate::models::ProjectKey, repository: String, repository_type: crate::models::PackageType, action: crate::models::PackageRepositoryEventAction, package_info: crate::models::PackageVersionRef) -> SPackageRepositoryWebhookEvent {
        SPackageRepositoryWebhookEvent {
            project_key: Box::new(project_key),
            repository,
            repository_type: Box::new(repository_type),
            action,
            package_info: Box::new(package_info),
        }
    }
}


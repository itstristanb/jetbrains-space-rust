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
pub struct ProjectsProjectAutomationDeploymentsStartPostRequest {
    #[serde(rename = "targetIdentifier")]
    pub target_identifier: Box<crate::models::TargetIdentifier>,
    #[serde(rename = "deploymentIdentifier", skip_serializing_if = "Option::is_none")]
    pub deployment_identifier: Option<Box<crate::models::DeploymentIdentifier>>,
    #[serde(rename = "version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub version: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "commitRefs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub commit_refs: Option<Option<Vec<crate::models::DeploymentCommitReference>>>,
    #[serde(rename = "externalLink", skip_serializing_if = "Option::is_none")]
    pub external_link: Option<Box<crate::models::ExternalLink>>,
    #[serde(rename = "syncWithAutomationJob", skip_serializing_if = "Option::is_none")]
    pub sync_with_automation_job: Option<bool>,
}

impl ProjectsProjectAutomationDeploymentsStartPostRequest {
    pub fn new(target_identifier: crate::models::TargetIdentifier) -> ProjectsProjectAutomationDeploymentsStartPostRequest {
        ProjectsProjectAutomationDeploymentsStartPostRequest {
            target_identifier: Box::new(target_identifier),
            deployment_identifier: None,
            version: None,
            description: None,
            commit_refs: None,
            external_link: None,
            sync_with_automation_job: None,
        }
    }
}



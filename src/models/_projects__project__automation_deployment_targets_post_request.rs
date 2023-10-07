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
pub struct ProjectsProjectAutomationDeploymentTargetsPostRequest {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "repositories", skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<crate::models::DeployTargetRepositoryDto>>,
    #[serde(rename = "manualControl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub manual_control: Option<Option<bool>>,
    #[serde(rename = "singleScheduled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub single_scheduled: Option<Option<bool>>,
    #[serde(rename = "hangTimeoutMinutes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hang_timeout_minutes: Option<Option<i32>>,
    #[serde(rename = "failTimeoutMinutes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fail_timeout_minutes: Option<Option<i32>>,
    #[serde(rename = "responsibleUsers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub responsible_users: Option<Option<Vec<String>>>,
    #[serde(rename = "responsibleTeams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub responsible_teams: Option<Option<Vec<String>>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Vec<crate::models::DeployTargetLink>>>,
    #[serde(rename = "customFields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Option<Vec<crate::models::CustomFieldInputValue>>>,
}

impl ProjectsProjectAutomationDeploymentTargetsPostRequest {
    pub fn new(key: String, name: String, description: String) -> ProjectsProjectAutomationDeploymentTargetsPostRequest {
        ProjectsProjectAutomationDeploymentTargetsPostRequest {
            key,
            name,
            description,
            repositories: None,
            manual_control: None,
            single_scheduled: None,
            hang_timeout_minutes: None,
            fail_timeout_minutes: None,
            responsible_users: None,
            responsible_teams: None,
            links: None,
            custom_fields: None,
        }
    }
}


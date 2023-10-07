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
pub struct RdDevConfiguration {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "repoConnections")]
    pub repo_connections: Vec<crate::models::RepoConnectionWithBranch>,
    #[serde(rename = "devContainer")]
    pub dev_container: Box<crate::models::RdDevContainer>,
    #[serde(rename = "ide")]
    pub ide: Box<crate::models::RdDevConfigurationIde>,
    #[serde(rename = "instanceTypeName")]
    pub instance_type_name: String,
    #[serde(rename = "project")]
    pub project: Box<crate::models::PrProject>,
    #[serde(rename = "hotPool", skip_serializing_if = "Option::is_none")]
    pub hot_pool: Option<Box<crate::models::DevConfigurationHotPool>>,
    #[serde(rename = "warmup")]
    pub warmup: Box<crate::models::DevConfigurationWarmup>,
    #[serde(rename = "hibernation", skip_serializing_if = "Option::is_none")]
    pub hibernation: Option<Box<crate::models::DevConfigurationHibernation>>,
    #[serde(rename = "projectRoot", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_root: Option<Option<String>>,
    #[serde(rename = "sshEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ssh_enabled: Option<Option<bool>>,
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<crate::models::DevConfigurationAccessSettingsDto>>,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl RdDevConfiguration {
    pub fn new(id: String, name: String, repo_connections: Vec<crate::models::RepoConnectionWithBranch>, dev_container: crate::models::RdDevContainer, ide: crate::models::RdDevConfigurationIde, instance_type_name: String, project: crate::models::PrProject, warmup: crate::models::DevConfigurationWarmup, archived: bool) -> RdDevConfiguration {
        RdDevConfiguration {
            id,
            name,
            repo_connections,
            dev_container: Box::new(dev_container),
            ide: Box::new(ide),
            instance_type_name,
            project: Box::new(project),
            hot_pool: None,
            warmup: Box::new(warmup),
            hibernation: None,
            project_root: None,
            ssh_enabled: None,
            access: None,
            archived,
        }
    }
}



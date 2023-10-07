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
pub struct DeploymentTargetM2ChannelInfo {
    #[serde(rename = "notificationDefaults")]
    pub notification_defaults: Box<crate::models::ChannelSpecificDefaults>,
    #[serde(rename = "deployTarget")]
    pub deploy_target: Box<crate::models::DeployTargetRecord>,
    #[serde(rename = "project")]
    pub project: Box<crate::models::PrProject>,
}

impl DeploymentTargetM2ChannelInfo {
    pub fn new(notification_defaults: crate::models::ChannelSpecificDefaults, deploy_target: crate::models::DeployTargetRecord, project: crate::models::PrProject) -> DeploymentTargetM2ChannelInfo {
        DeploymentTargetM2ChannelInfo {
            notification_defaults: Box::new(notification_defaults),
            deploy_target: Box::new(deploy_target),
            project: Box::new(project),
        }
    }
}


